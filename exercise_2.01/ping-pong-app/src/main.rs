use actix_web::{get, web, App, HttpServer};
use serde::Deserialize;
use std::{
    env::var,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

struct State {
    counter: Arc<AtomicUsize>,
}

#[derive(Deserialize)]
struct PingPongQuery {
    #[serde(default)]
    no_increment: bool,
}

#[get("/pingpong")]
async fn pong(state: web::Data<State>, query: web::Query<PingPongQuery>) -> String {
    let number = match query.no_increment {
        true => state.counter.load(Ordering::SeqCst),
        false => state.counter.fetch_add(1, Ordering::SeqCst),
    };

    format!("pong {}", number)
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

    let counter = Arc::new(AtomicUsize::new(0));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                counter: counter.clone(),
            }))
            .service(pong)
    })
    .bind(("0.0.0.0", port))?;

    println!("HTTP server listening on port :{}", port);

    server.run().await
}
