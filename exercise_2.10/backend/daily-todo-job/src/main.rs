use todo_common::{config, db};
use tokio_postgres::types::ToSql;

#[tokio::main]
async fn main() {
    let config = config::load_config().unwrap();
    let db = db::DbConnection::connect(&config.database).await.unwrap();
    let client = db.get_client().await.unwrap();

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let res = http_client
        .get("https://en.wikipedia.org/wiki/Special:Random")
        .send()
        .await
        .unwrap();

    let link = res.headers().get("location").unwrap();

    let id = uuid::Uuid::new_v4();

    let stmt = client
        .prepare("INSERT INTO todos (id, text) VALUES ($1, $2)")
        .await
        .unwrap();

    let text = format!("Read: {}", link.to_str().unwrap());

    client
        .execute(&stmt, &[&id as &(dyn ToSql + Sync), &text])
        .await
        .unwrap();

    println!("Created item {} for article {}", id, link.to_str().unwrap());
}
