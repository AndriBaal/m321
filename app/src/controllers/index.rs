use crate::{
    app::AppState,
    controllers::auth::AuthRequired,
    views::{context::Context, index::IndexView},
};
use actix_session::Session;
use actix_web::{
    Responder, get, post,
    web::{self, Data},
};
use bson::{doc, oid::ObjectId};

#[get("/", wrap = "AuthRequired")]
async fn index(app: Data<AppState>, session: Session) -> impl Responder {
    // let user_id = app.user_id(&session);
    // let tasks = app
    //     .tasks
    //     .find(doc! {
    //         "user_id": user_id
    //     })
    //     .await
    //     .unwrap()
    //     .try_collect::<Vec<_>>()
    //     .await
    //     .unwrap();

    // let mut tasks = tasks;
    // tasks.sort_by_key(|task| task.done);

    app.render_template(IndexView {
        ctx: Context::new(&app, session),
    })
}
