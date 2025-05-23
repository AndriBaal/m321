use actix_session::Session;
use uuid::Uuid;

use crate::app::AppState;

pub struct Context {
    pub user_id: Option<Uuid>,
}

impl Context {
    pub fn new(_app: &AppState, session: Session) -> Self {
        let user_id = session.get::<Uuid>("user_id").unwrap();
        Self { user_id }
    }
}
