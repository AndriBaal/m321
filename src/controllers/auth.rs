use std::future::{Ready, ready};

use crate::{
    app::AppState,
    views::{
        auth::{Login, SignUp},
        context::Context,
    },
};

use actix_session::{Session, SessionExt};
use actix_web::{
    Error, Responder,
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    get, post,
    web::{self, Data},
};
use bson::{doc, oid::ObjectId};
use futures::future::LocalBoxFuture;
use serde::Deserialize;

#[derive(Deserialize)]
struct UserForm {
    username: String,
    password: String,
}

#[get("/login")]
async fn login_get(app: web::Data<AppState>, session: Session) -> impl Responder {
    app.render_template(Login {
        ctx: Context::new(&app, session),
    })
}

#[get("/logout")]
async fn logout(app: web::Data<AppState>, session: Session) -> impl Responder {
    session.remove("user_id");
    return app.redirect("/");
}

// #[post("/login")]
// async fn login_post(
//     app: web::Data<AppState>,
//     web::Form(form): web::Form<UserForm>,
//     session: Session,
// ) -> impl Responder {
//     // let filter = doc! { "username": form.username, "password": form.password };
//     // if let Some(user) = app.users.find_one(filter).await.unwrap() {
//     //     session.insert("user_id", user.id.unwrap()).unwrap();
//     //     return app.redirect("/");
//     // } else {
//     //     return app.render_template(Login {
//     //         ctx: Context::new(&app, session),
//     //     });
//     // }
// }

// #[get("/signup")]
// async fn signup_get(app: web::Data<AppState>, session: Session) -> impl Responder {
//     app.render_template(SignUp {
//         ctx: Context::new(&app, session),
//     })
// }

// #[post("/signup")]
// async fn signup_post(
//     app: web::Data<AppState>,
//     web::Form(form): web::Form<UserForm>,
// ) -> impl Responder {
//     app.users
//         .insert_one(User {
//             id: None,
//             username: form.username,
//             password: form.password,
//         })
//         .await
//         .unwrap();
//     return app.redirect("/login");
// }

// // pub async fn my_middleware(
// //     req: ServiceRequest,
// //     next: Next<BoxBody>,
// // ) -> Result<ServiceResponse<BoxBody>, Error> {

// //     let session = req.get_session(); // Get session
// //     let app = req.app_data::<Data<AppState>>().cloned().unwrap(); // Get shared AppState

// //     return if let Some(_user_id) = app.validate_session(&session) {
// //         next.call(req).await
// //     } else {
// //         let (req, _) = req.into_parts();
// //         Ok(ServiceResponse::new(req, app.redirect("/login")))

// //         // return Ok(app.redirect("/login"))
// //     };
// // }

pub struct AuthRequired;
impl<S> Transform<S, ServiceRequest> for AuthRequired
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthRequiredMiddleWare<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthRequiredMiddleWare { service }))
    }
}

pub struct AuthRequiredMiddleWare<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthRequiredMiddleWare<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session(); // Get session
        let app = req.app_data::<Data<AppState>>().cloned().unwrap(); // Get shared AppState

        if let Some(_user_id) = session.get::<ObjectId>("user_id").unwrap() {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        } else {
            let (req, _) = req.into_parts();
            return Box::pin(async move { Ok(ServiceResponse::new(req, app.redirect("/login"))) });
        };
    }
}
