use std::{ future::{ ready, Ready }, string };

use crate::{ app::{ AppState, Args }, views::{ auth::{ Login, SignUp }, context::Context } };

use actix_session::{ Session, SessionExt };
use actix_web::{
    body::BoxBody, dev::{ forward_ready, ConnectionInfo, Service, ServiceRequest, ServiceResponse, Transform }, get, middleware::Logger, post, web::{ self, Data }, Error, HttpRequest, HttpResponse, Responder
};
use bson::{ doc, oid::ObjectId };
use futures::future::LocalBoxFuture;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, RedirectUrl, RevocationErrorResponseType, Scope, TokenUrl
};
use oauth2::{
    basic::{ BasicErrorResponseType, BasicTokenType },
    EmptyExtraTokenFields,
    StandardErrorResponse,
    StandardRevocableToken,
    StandardTokenIntrospectionResponse,
    StandardTokenResponse,
    EndpointSet,
    EndpointNotSet,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct UserForm {
    username: String,
    password: String,
}

#[get("/login")]
async fn login_get(app: web::Data<AppState>, session: Session) -> impl Responder {
    HttpResponse::Ok().body("test 1")
    // app.render_template(Login {
    //     ctx: Context::new(&app, session),
    // })
}

#[get("/logincallback")]
async fn login_get_callback(app: web::Data<AppState>, session: Session, req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("test 2")
    // let query = web::Query::<std::collections::HashMap<String, String>>::from_query(req.query_string()).unwrap();
	// let code = query.get("code").unwrap().to_string();

    // let connection_info = req.connection_info().clone();
	// let client = oauth_client(&app.args, connection_info);

	// let token_result = client
	// 	.exchange_code(AuthorizationCode::new(code))
	// 	.request_async(async_http_client)
	// 	.await;

	// match token_result {
	// 	Ok(token) => {
	// 		let access_token = token.access_token().secret();
	// 		HttpResponse::Ok().body(format!("Access token: {}", access_token))
	// 	}
	// 	Err(err) => HttpResponse::InternalServerError().body(format!("Auth error: {:?}", err))
	// }
    // app.render_template(Login {
    //     ctx: Context::new(&app, session),
    // })
}

#[get("logout")]
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

fn oauth_client(
    args: &Args,
) -> Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet
> {
    let keycloak_realm = &args.keycloak_realm;
    let keycloak_port = &args.keycloak_port;
    let keycloak_secret = std::fs
        ::read_to_string(&args.keycloak_secret_file)
        .expect("Cannot read secret");
    let keycloak_client_id = &args.keycloak_client_id;

    let scheme = connection_info.scheme();
    let host = connection_info.host(); // e.g. "localhost:80"

    // Strip port from host if present
    let host_without_port = host.split(':').next().unwrap_or(host);

    let current_root = format!("{}://{}", scheme, host);
    let redirect_uri = format!("{}/logincallback", current_root);

    let keycloak_root = format!("{}://{}:{}", scheme, host_without_port, keycloak_port);

    let client = BasicClient::new(ClientId::new(keycloak_client_id.clone()))
        .set_client_secret(ClientSecret::new(keycloak_secret))
        .set_auth_uri(
            AuthUrl::new(
                format!("{}/realms/{}/protocol/openid-connect/auth", keycloak_root, keycloak_realm)
            ).expect("Invalid Auth URL")
        )
        .set_token_uri(
            TokenUrl::new(
                format!("{}/realms/{}/protocol/openid-connect/token", keycloak_root, keycloak_realm)
            ).expect("Invalid Token URL")
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("Invalid redirect URI"));

    return client;
}

pub struct AuthRequired;
impl<S> Transform<S, ServiceRequest>
    for AuthRequired
    where
        S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
        S::Future: 'static
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

impl<S> Service<ServiceRequest>
    for AuthRequiredMiddleWare<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
        S::Future: 'static
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let app = req.app_data::<Data<AppState>>().cloned().expect("AppState missing from request");

        // ✅ Session gültig → weiterleiten
        if let Ok(Some(_user_id)) = session.get::<ObjectId>("user_id") {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        let client = oauth_client(&app.args);
        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        // 🔥 Achtung: Jetzt erst `into_parts()` aufrufen
        let (req_head, _pl) = req.into_parts();

        Box::pin(async move {
            Ok(
                ServiceResponse::new(
                    req_head,
                    HttpResponse::Found()
                        .insert_header(("Location", auth_url.to_string()))
                        .finish()
                        .map_into_boxed_body()
                )
            )
        })
    }
}
