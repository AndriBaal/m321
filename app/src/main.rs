pub mod app;
pub mod controllers;
pub mod models;
pub mod tests;
pub mod views;

use app::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState::new().await.unwrap();
    models::setup_models(&state).await;
    views::setup_views(&state).await;
    controllers::setup_server(state).await
}
