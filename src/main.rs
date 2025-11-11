mod api;
mod db;
mod model;
mod repo;

use crate::api::api_routes;
use anyhow::Result as AnyResult;
use axum::Router;
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> AnyResult<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    db::global_db().await; // 提前初始化连接得了
    let app = Router::new().nest("/api/v1", api_routes());
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
