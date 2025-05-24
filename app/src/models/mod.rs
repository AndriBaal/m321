use crate::app::AppState;

pub mod session;
pub mod temperature_log;

pub async fn setup_models(app: &AppState) {
    if app.args.rebuild_indexes {
        session::setup_index(app).await;
    }
}
