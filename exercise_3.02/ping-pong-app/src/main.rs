use actix_web::{get, web, App, HttpResponse, HttpServer};
use deadpool::managed::Pool;
use deadpool_postgres::Manager;
use serde::Deserialize;
use std::{
    env::{var, var_os},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

fn default_port() -> u16 {
    8080
}

#[derive(Deserialize)]
struct ServerConfig {
    port: u16,
}

#[derive(Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    database: String,
    user: String,
    password: String,
}

#[derive(Deserialize)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

fn load_config() -> anyhow::Result<AppConfig> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .add_source(config::Environment::with_prefix("PONG").separator("_"))
        .build()?;

    let app_config = settings.try_deserialize()?;

    Ok(app_config)
}

struct State {
    pool: Pool<Manager>,
}

#[derive(Deserialize)]
struct PingPongQuery {
    #[serde(default)]
    no_increment: bool,
}

#[get("/")]
async fn healthcheck(state: web::Data<State>) -> HttpResponse {
    let client_result = state.pool.get().await;

    if client_result.is_err() {
        return HttpResponse::InternalServerError().body("Database Connection Error");
    }

    return HttpResponse::Ok().body("OK");
}

#[get("/pingpong")]
async fn pong(state: web::Data<State>, query: web::Query<PingPongQuery>) -> String {
    let client = state.pool.get().await.unwrap();

    let value: i32 = match query.no_increment {
        true => {
            let stmt = client
                .prepare("SELECT value FROM pong_counter LIMIT 1")
                .await
                .unwrap();

            client
                .query(&stmt, &[])
                .await
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .get(0)
        }
        false => {
            let stmt = client
                .prepare("UPDATE pong_counter SET value = value + 1 RETURNING value")
                .await
                .unwrap();

            client
                .query(&stmt, &[])
                .await
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .get(0)
        }
    };

    format!("pong {}", value)
}

async fn create_pool(config: &DatabaseConfig) -> anyhow::Result<Pool<Manager>> {
    let mut cfg = deadpool_postgres::Config::new();

    cfg.host = Some(config.host.clone());
    cfg.port = Some(config.port);
    cfg.dbname = Some(config.database.clone());
    cfg.user = Some(config.user.clone());
    cfg.password = Some(config.password.clone());

    let pool = cfg.create_pool(
        Some(deadpool_postgres::Runtime::Tokio1),
        tokio_postgres::NoTls,
    )?;

    Ok(pool)
}

async fn bootstrap(pool: &Pool<Manager>) -> anyhow::Result<()> {
    let client = pool.get().await?;

    let stmt = client
        .prepare("CREATE TABLE IF NOT EXISTS pong_counter ( value INT NOT NULL )")
        .await?;

    client.execute(&stmt, &[]).await?;

    let stmt = client
        .prepare(
            "CREATE UNIQUE INDEX IF NOT EXISTS pong_counter_singleton ON pong_counter ((true))",
        )
        .await?;

    client.execute(&stmt, &[]).await?;

    let stmt = client
        .prepare("INSERT INTO pong_counter (value) VALUES (0)")
        .await?;

    let res = client.execute(&stmt, &[]).await; // If this fails, let it fail. :shrug:

    if let Err(e) = res {
        eprintln!("Inserting default counter value failed: {}", e);
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = load_config().unwrap();

    let pool = create_pool(&config.database).await.unwrap();

    bootstrap(&pool).await.unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State { pool: pool.clone() }))
            .service(pong)
            .service(healthcheck)
    })
    .bind(("0.0.0.0", config.server.port))?;

    println!("HTTP server listening on port :{}", config.server.port);

    server.run().await
}
