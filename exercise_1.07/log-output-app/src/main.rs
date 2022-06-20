use actix_web::{get, web, App, HttpServer};
use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use std::env::var;

#[get("/status")]
async fn status(state: web::Data<State>) -> String {
    format!("{} {}", state.timestamp, state.random_string)
}

struct State {
    random_string: String,
    timestamp: DateTime<Utc>,
}

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

    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let timestamp = Utc::now();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                random_string: random_string.clone(),
                timestamp,
            }))
            .service(status)
    })
    .bind(("0.0.0.0", port))?;

    println!("HTTP server listening on port :{}", port);

    server.run().await
}
