use actix_web::{
    get, post,
    web::{Data, Json},
    App, HttpServer, Result,
};
use chrono::{Date, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env::{var, var_os},
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::SystemTime,
};
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
async fn create_todo(body: Json<CreateTodoPayload>, state: Data<Arc<Mutex<State>>>) -> Json<Todo> {
    let mut state = state.lock().unwrap();
    let todo = Todo::new(body.text.clone());
    state.todos.insert(todo.id.clone(), todo.clone());

    Json(todo)
}

#[get("/api/todo")]
async fn list_todos(state: Data<Arc<Mutex<State>>>) -> Json<Vec<Todo>> {
    let state = state.lock().unwrap();
    Json(state.todos.values().cloned().collect())
}

#[get("/api/picture-of-the-day")]
async fn picture_of_the_day(config: Data<Config>) -> Json<PictureOfTheDayResponse> {
    let date: Date<Utc> = Utc::today();
    let today = date.format("%Y-%m-%d").to_string();
    let picture_path = config.data_path.join(format!("{}.jpg", today));

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
struct Config {
    data_path: PathBuf,
}

struct State {
    todos: HashMap<Uuid, Todo>,
}

impl Default for State {
    fn default() -> Self {
        State {
            todos: HashMap::new(),
        }
    }
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

    let data_path = var_os("DATA_PATH")
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or("/tmp".into())
        .parse::<PathBuf>()
        .unwrap();

    let config = Config { data_path };

    let state = Arc::new(Mutex::new(State::default()));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(state.clone()))
            .service(picture_of_the_day)
            .service(create_todo)
            .service(list_todos)
            .service(actix_files::Files::new("/data", config.data_path.clone()))
            .default_service(
                actix_files::Files::new("", "/static").index_file("/static/index.html"),
            )
    })
    .bind(("0.0.0.0", port))?;

    println!("HTTP server listening on port :{}", port);

    server.run().await
}
