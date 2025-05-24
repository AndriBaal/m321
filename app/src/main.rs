pub mod app;
pub mod controllers;
pub mod models;
pub mod mqtt_client;
pub mod tests;
pub mod views;

use app::AppState;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let state = AppState::new().await;
    models::setup_models(&state).await;
    views::setup_views(&state).await;
    controllers::setup_server(state).await
}
