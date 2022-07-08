use actix_web::{
    get,
    http::header::ContentType,
    middleware::Logger,
    post,
    web::{Data, Json},
    App, HttpResponse, HttpServer, Result,
};
use chrono::{Date, NaiveDateTime, Utc};
use db::DbConnection;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::Path, time::SystemTime};
use todo_common::{config, db};
use tokio_postgres::types::ToSql;
use uuid::Uuid;

#[derive(Serialize)]
struct PictureOfTheDayResponse {
    url: String,
    last_updated: String,
}

async fn fetch_picture<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let path = path.as_ref();

    let bytes = reqwest::get("https://picsum.photos/1200.jpg")
        .await?
        .bytes()
        .await?;

    let mut file = File::create(path)?;
    file.write(&*bytes)?;

    Ok(())
}

#[derive(Clone, Debug, Serialize)]
struct Todo {
    id: Uuid,
    text: String,
}

impl Todo {
    fn new(text: String) -> Todo {
        let id = Uuid::new_v4();

        Todo { id, text }
    }
}

#[derive(Deserialize)]
struct CreateTodoPayload {
    text: String,
}

#[post("/api/todo")]
async fn create_todo(body: Json<CreateTodoPayload>, state: Data<State>) -> HttpResponse {
    let client = state.db.get_client().await.unwrap();

    if body.text.len() > 140 {
        log::warn!("Todo entry too long: {}", body.text);

        return HttpResponse::BadRequest().finish();
    }

    let todo = Todo::new(body.text.clone());

    let stmt = client
        .prepare("INSERT INTO todos (id, text) VALUES ($1, $2)")
        .await
        .unwrap();

    client
        .execute(&stmt, &[&todo.id as &(dyn ToSql + Sync), &todo.text])
        .await
        .unwrap();

    log::info!("Created todo item {}: {}", todo.id, todo.text);

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(todo)
}

#[get("/api/todo")]
async fn list_todos(state: Data<State>) -> Json<Vec<Todo>> {
    let client = state.db.get_client().await.unwrap();
    let stmt = client.prepare("SELECT id, text FROM todos").await.unwrap();
    let todos = client
        .query(&stmt, &[])
        .await
        .unwrap()
        .iter()
        .map(|row| Todo {
            id: row.get(0),
            text: row.get(1),
        })
        .collect();

    Json(todos)
}

#[get("/api/picture-of-the-day")]
async fn picture_of_the_day(config: Data<config::Config>) -> Json<PictureOfTheDayResponse> {
    let date: Date<Utc> = Utc::today();
    let today = date.format("%Y-%m-%d").to_string();
    let picture_path = config.app.data_path.join(format!("{}.jpg", today));

    if !picture_path.exists() {
        fetch_picture(&picture_path).await.unwrap();
    }

    let since_epoch = picture_path
        .metadata()
        .unwrap()
        .modified()
        .unwrap()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let last_updated = NaiveDateTime::from_timestamp(
        since_epoch.as_secs().try_into().unwrap(),
        since_epoch.subsec_nanos(),
    );

    Json(PictureOfTheDayResponse {
        url: format!("/data/{}.jpg", today),
        last_updated: last_updated.format("%Y-%m-%d").to_string(),
    })
}

#[derive(Clone)]
struct State {
    db: DbConnection,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let config = config::load_config().unwrap();

    println!("{:?}", config);

    let config_clone = config.clone();

    let db = DbConnection::connect(&config.database).await.unwrap();

    let migrations = db.run_migrations().await.unwrap();

    for migration in migrations.applied_migrations() {
        println!(
            "Applied migration {}{}: {}",
            migration.prefix(),
            migration.version(),
            migration.name()
        );
    }

    let state = State { db };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config_clone.clone()))
            .app_data(Data::new(state.clone()))
            .service(picture_of_the_day)
            .service(create_todo)
            .service(list_todos)
            .service(actix_files::Files::new(
                "/data",
                config.app.data_path.clone(),
            ))
            .default_service(
                actix_files::Files::new("", "/static").index_file("/static/index.html"),
            )
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", config.http.port))?;

    println!("HTTP server listening on port :{}", config.http.port);

    server.run().await
}
