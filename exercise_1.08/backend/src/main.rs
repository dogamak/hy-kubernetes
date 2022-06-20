use actix_web::{App, HttpServer, Result};
use std::env::var;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let port_str = match var("PORT") {
        Err(_) => "8080".into(),
        Ok(s) => s,
    };

    let port = match port_str.parse::<u16>() {
        Ok(port) => port,
        Err(_) => {
            eprintln!("Invalid port number!");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "invalid port number",
            ));
        }
    };

    let server = HttpServer::new(|| {
        App::new().default_service(
            actix_files::Files::new("", "/static").index_file("/static/index.html"),
        )
    })
    .bind(("0.0.0.0", port))?;

    println!("HTTP server listening on port :{}", port);

    server.run().await
}
