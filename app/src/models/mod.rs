use crate::app::AppState;

pub mod session;
pub mod device_log;

pub async fn setup_models(app: &AppState) {
    if app.args.rebuild_indexes {
        session::setup_index(app).await;
    }
}
