use actix_web::{
    get,
    http::{uri::PathAndQuery, Uri},
    web, App, HttpServer,
};
use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use std::env::{var, var_os};

#[get("/status")]
async fn status(state: web::Data<State>) -> String {
    let mut url_parts = state.pong_service_url.clone().into_parts();
    url_parts.path_and_query = Some(PathAndQuery::from_static("/pingpong?no_increment=true"));
    let url = Uri::from_parts(url_parts).unwrap().to_string();

    let pongs = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .split(" ")
        .skip(1)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    format!(
        "{}\n{} {}\nPing / Pongs: {}",
        state.message, state.timestamp, state.random_string, pongs
    )
}

#[derive(Clone)]
struct State {
    random_string: String,
    timestamp: DateTime<Utc>,
    message: String,
    pong_service_url: Uri,
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

    let message = var_os("MESSAGE")
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or("<No Message>".into());

    let pong_service_url = var_os("PONG_URL")
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or("http://localhost:8088".into())
        .parse::<Uri>()
        .unwrap();

    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let timestamp = Utc::now();

    let state = State {
        random_string,
        timestamp,
        message,
        pong_service_url,
    };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(status)
    })
    .bind(("0.0.0.0", port))?;

    println!("HTTP server listening on port :{}", port);

    server.run().await
}
