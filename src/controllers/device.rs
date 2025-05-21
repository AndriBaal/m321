use crate::{
    app::AppState,
    controllers::auth::AuthRequired,
    models::{device::Device},
    views::{context::Context, device::DeviceView},
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

    app.render_template(DeviceView {
        ctx: Context::new(&app, session),
    })
}

#[post("/update", wrap = "AuthRequired")]
async fn update(
    app: Data<AppState>,
    session: Session,
    // web::Form(form): web::Form<TaskIdForm>,
) -> impl Responder {
    // let user_id = app.user_id(&session);

    // if let Some(task) = app
    //     .tasks
    //     .find_one(doc! {"_id": form.task_id, "user_id": user_id})
    //     .await
    //     .unwrap()
    // {
    //     let new_done_status = !task.done;
    //     app.tasks
    //         .update_one(
    //             doc! {"_id": form.task_id},
    //             doc! {"$set": {"done": new_done_status}},
    //         )
    //         .await
    //         .unwrap();
    // }

    return app.redirect("/");
}
