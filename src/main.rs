use std::env;

use axum::{self, routing::get, Router};
use dotenv::dotenv;

use crate::store::Db;

pub mod custom_error;
pub mod store;

#[tokio::main()]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").unwrap();

    let mut db = Db::new(&db_url).await;
    let _ = db.migrate().await;

    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello World"
}
//sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;
