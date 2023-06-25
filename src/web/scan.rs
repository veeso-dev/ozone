use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use futures_util::{StreamExt, TryStreamExt};

use crate::clamav::{ClamAvClient, Scan};

use super::WebserverData;

#[derive(Serialize, Debug)]
struct ScanResponse {
    files: Vec<ScannedFile>,
}

#[derive(Serialize, Debug)]
struct ScannedFile {
    name: String,
    safe: bool,
    size: usize,
}

#[post("/scan")]
async fn scan(
    mut payload: Multipart,
    data: web::Data<WebserverData>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let content_length: usize = match request.headers().get("content-length") {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap_or(0),
        None => 0,
    };

    debug!("content length is {content_length}");
    match content_length {
        0 => return Ok(HttpResponse::BadRequest().finish()),
        length if length > data.max_upload_size => {
            return Ok(HttpResponse::BadRequest().body(format!(
            "The uploaded file is too large. Maximum size is {} bytes, but file has size {length}.",
            data.max_upload_size
        )))
        }
        _ => {}
    };

    // iterate over files
    let mut files = vec![];
    debug!("scanning files...");

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .unwrap_or_default()
            .to_string();
        debug!("processing file {filename}");

        let mut clamav_client = match ClamAvClient::init(&data.clamav_address).await {
            Ok(client) => client,
            Err(err) => {
                error!("failed to init clamav client: {err}");
                return Ok(HttpResponse::InternalServerError().body(format!("failed to scan file")));
            }
        };

        let mut total_size = 0;
        while let Some(Ok(chunk)) = field.next().await {
            total_size += chunk.len();
            if let Err(err) = clamav_client.send(chunk).await {
                error!("failed to send data to clamav: {err}");
                return Ok(HttpResponse::InternalServerError().body(format!("failed to scan file")));
            }
        }

        // wait for response
        let is_safe = match clamav_client.finish().await {
            Ok(Scan::Safe) => true,
            Ok(Scan::Unsafe) => false,
            Err(err) => {
                error!("failed to read clamav response: {err}");
                return Ok(HttpResponse::InternalServerError().body(format!("failed to scan file")));
            }
        };

        files.push(ScannedFile {
            name: filename,
            safe: is_safe,
            size: total_size,
        });
    }

    debug!("{} files scanned", files.len());

    Ok(HttpResponse::Ok().json(ScanResponse { files }))
}
