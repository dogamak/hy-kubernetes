use actix_web::{get, web, App, HttpServer};
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

#[get("/pingpong")]
async fn pong(state: web::Data<State>) -> String {
    format!("pong {}", state.counter.fetch_add(1, Ordering::SeqCst))
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

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                counter: Arc::new(AtomicUsize::new(0)),
            }))
            .service(pong)
    })
    .bind(("0.0.0.0", port))?;

    println!("HTTP server listening on port :{}", port);

    server.run().await
}
