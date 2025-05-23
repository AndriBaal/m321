pub mod app;
pub mod controllers;
pub mod models;
pub mod tests;
pub mod views;
pub mod mqtt_client;

use app::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState::new().await;
    models::setup_models(&state).await;
    views::setup_views(&state).await;
    controllers::setup_server(state).await
}
