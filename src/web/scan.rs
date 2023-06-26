use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use futures_util::{StreamExt, TryStreamExt};
use serde_with::skip_serializing_none;
use tempfile::tempfile;
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};

use super::WebserverData;
use crate::clamav::{ClamAvClient, Scan};

#[derive(Serialize, Debug)]
struct ScanResponse {
    files: Vec<ScannedFile>,
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
struct ScannedFile {
    name: String,
    filename: String,
    safe: bool,
    size: usize,
    threat: Option<String>,
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
        0 => {
            error!("content length is 0");
            return Ok(HttpResponse::BadRequest().finish());
        }
        length if length > data.max_upload_size => {
            error!("The uploaded file is too large. Maximum size is {} bytes, but file has size {length}.", data.max_upload_size);
            return Ok(HttpResponse::BadRequest().body(format!(
            "The uploaded file is too large. Maximum size is {} bytes, but file has size {length}.",
            data.max_upload_size
        )));
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
                return Ok(
                    HttpResponse::InternalServerError().body("failed to scan file".to_string())
                );
            }
        };

        let temp = tempfile()?;
        let mut async_temp = File::from_std(temp);
        let mut total_size = 0;

        while let Some(Ok(chunk)) = field.next().await {
            total_size += chunk.len();
            async_temp.write_all(&chunk).await?;
        }

        // rewind file
        async_temp.seek(std::io::SeekFrom::Start(0)).await?;

        // scan file
        let (is_safe, threat) = match clamav_client.scan(async_temp, total_size as u32).await {
            Ok(Scan::Safe) => (true, None),
            Ok(Scan::Unsafe(threat_name)) => (false, Some(threat_name)),
            Err(err) => {
                error!("failed to read clamav response: {err}");
                return Ok(
                    HttpResponse::InternalServerError().body("failed to scan file".to_string())
                );
            }
        };

        debug!("scan result for {filename}: is safe? {is_safe}");

        files.push(ScannedFile {
            name: field.name().to_string(),
            filename,
            safe: is_safe,
            size: total_size,
            threat,
        });
    }

    debug!("{} files scanned", files.len());

    Ok(HttpResponse::Ok().json(ScanResponse { files }))
}
