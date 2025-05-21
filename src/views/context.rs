use actix_session::Session;
use bson::oid::ObjectId;

use crate::app::AppState;

pub struct Context {
    pub user_id: Option<ObjectId>,
}

impl Context {
    pub fn new(_app: &AppState, session: Session) -> Self {
        let user_id = session.get::<ObjectId>("user_id").unwrap();
        Self { user_id }
    }
}
