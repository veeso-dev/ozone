use actix_web::{dev::Server, web::Data, App as ActixApp, HttpServer};
use std::net::TcpListener;

mod health_check;
mod scan;

pub struct WebServer {
    server: Server,
}

struct WebserverData {
    pub clamav_address: String,
    pub max_upload_size: usize,
}

impl WebServer {
    /// Initialize web server
    pub async fn init(
        clamav_address: &str,
        max_upload_size: usize,
        web_port: u16,
    ) -> anyhow::Result<Self> {
        debug!("webserver initialized");
        debug!("protobuf url: {clamav_address}");
        debug!("web port: {web_port}");

        let listener = TcpListener::bind(&format!("0.0.0.0:{web_port}"))?;

        let server = {
            let clamav_address = clamav_address.to_string();
            HttpServer::new(move || {
                let web_data = Data::new(WebserverData {
                    clamav_address: clamav_address.to_string(),
                    max_upload_size,
                });
                ActixApp::new()
                    .service(health_check::check_action)
                    .service(scan::scan)
                    .app_data(web_data)
            })
            .listen(listener)?
            .run()
        };

        info!("web server initialized");
        Ok(Self { server })
    }

    /// Run web server
    pub async fn run(self) -> anyhow::Result<()> {
        info!("running web server");
        self.server.await?;
        info!("web server stopped");
        Ok(())
    }
}
