use std::{
    fs,
    future::{Ready, ready},
    str::FromStr,
};

use crate::{
    app::{AppState, Args},
    views::{context::Context, login::LoginView},
};

use actix_session::{Session, SessionExt};
use actix_web::{
    Error, HttpRequest, HttpResponse, Responder,
    body::BoxBody,
    dev::{ConnectionInfo, Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    get,
    web::{self, Data},
};
use bson::doc;
use futures::future::LocalBoxFuture;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, decode};
use oauth2::{
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    RevocationErrorResponseType, Scope, TokenResponse, TokenUrl, basic::BasicClient,
};
use oauth2::{
    EmptyExtraTokenFields, EndpointNotSet, EndpointSet, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse,
    basic::{BasicErrorResponseType, BasicTokenType},
};
use serde::Deserialize;
use uuid::Uuid;

#[get("/login")]
async fn login(app: web::Data<AppState>, session: Session, req: HttpRequest) -> impl Responder {
    let query =
        web::Query::<std::collections::HashMap<String, String>>::from_query(req.query_string());
    let code = match query {
        Ok(q) => q.get("code").cloned(),
        Err(_) => None,
    };

    if code.is_none() {
        return HttpResponse::BadRequest().body("Missing code");
    }

    let code = AuthorizationCode::new(code.unwrap());
    let connection_info = req.connection_info().clone();
    let client = oauth_client(&app.args, &connection_info);

    let http_client = &reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(usize::max_value()))
        .build()
        .unwrap();

    let token_result = client.exchange_code(code).request_async(http_client).await;

    let token = match token_result {
        Ok(token) => token,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Token error: {:?}", err));
        }
    };

    let secret = token.access_token().secret();
    match extract_sub_from_jwt(&secret) {
        Some(id) => {
            log::info!("redirecting to main. found id: {}", id);
            session.insert("user_id", id).unwrap();
            return app.redirect("/");
        }
        None => {
            return HttpResponse::InternalServerError().body("Failed to get user ID");
        }
    };
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Claims {
    sub: String,
    exp: usize, // Ablaufzeit
}
fn extract_sub_from_jwt(token: &str) -> Option<Uuid> {
    let decoding_key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::HS256);
    validation.insecure_disable_signature_validation();
    let t: Result<TokenData<Claims>, jsonwebtoken::errors::Error> =
        decode(token, &decoding_key, &validation);
    if t.is_err() {
        return None;
    }
    let TokenData { claims, .. }: TokenData<Claims> = t.unwrap();

    match Uuid::from_str(&claims.sub) {
        Ok(id) => {
            return Some(id);
        }
        Err(_err) => {
            return None;
        }
    };
}

#[derive(serde::Deserialize)]
struct TokenResponseBody {
    access_token: String,
    // add other fields here if you need them…
}

#[get("logout")]
async fn logout(app: web::Data<AppState>, session: Session, _req: HttpRequest) -> impl Responder {
    let user_id = match session.get::<Uuid>("user_id") {
        Ok(Some(id)) => id.to_string(),
        _ => return app.redirect("/"), // nothing to do
    };
    session.clear();

    // TODO: nicht fertig

    // 2) Build a client‐credentials token request
    // let host = format!(
    //     "http://{}:{}",
    //     app.args.keycloak_external_host, app.args.keycloak_external_port,
    // );
    // let token_url = format!(
    //     "{}/realms/{}/protocol/openid-connect/token",
    //     host, app.args.keycloak_realm
    // );

    // let client_secret = fs::read_to_string(&app.args.keycloak_secret_file)
    //     .expect("Cannot read client secret file")
    //     .trim()
    //     .to_string();

    // let http = reqwest::Client::new();
    // let token_res = http
    //     .post(&token_url)
    //     .form(&[
    //         ("grant_type", "client_credentials"),
    //         ("client_id", &app.args.keycloak_client_id),
    //         ("client_secret", &client_secret),
    //     ])
    //     .send()
    //     .await
    //     .map_err(|e| {
    //         log::error!("Failed to fetch admin token: {}", e);
    //         HttpResponse::InternalServerError().body("Token request failed")
    //     })
    //     .unwrap();

    // if !token_res.status().is_success() {
    //     log::error!("Token endpoint returned {}", token_res.status());
    //     return HttpResponse::InternalServerError().body("Token request error");
    // }

    // let tok: TokenResponseBody = token_res
    //     .json()
    //     .await
    //     .map_err(|e| {
    //         log::error!("Failed to parse token JSON: {}", e);
    //         HttpResponse::InternalServerError().body("Bad token JSON")
    //     })
    //     .unwrap();

    // // 3) Call the Admin logout endpoint
    // let logout_url = format!(
    //     "{}/admin/realms/{}/users/{}/logout",
    //     host, app.args.keycloak_realm, user_id
    // );
    // let logout_res = http
    //     .post(&logout_url)
    //     .bearer_auth(&tok.access_token)
    //     .send()
    //     .await
    //     .map_err(|e| {
    //         log::error!("Logout request error: {}", e);
    //         HttpResponse::InternalServerError().body("Logout request failed")
    //     })
    //     .unwrap();

    // if !logout_res.status().is_success() {
    //     log::error!("Admin logout returned {}", logout_res.status());
    //     return HttpResponse::InternalServerError().body("Admin logout error");
    // }

    // 4) Finally, redirect the browser home
    return app.redirect("/");
}

fn oauth_client(
    args: &Args,
    connection_info: &ConnectionInfo,
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
    EndpointSet,
> {
    // 1) Redirect-URI für den Browser
    let redirect_uri = format!(
        "{}://{}:{}/login",
        connection_info.scheme(),
        connection_info.host(),
        args.nginx_port
    );
    log::info!("OAuth Redirect URI = {}", redirect_uri);

    // 2) AuthUrl: extern für den Browser
    let auth_uri = format!(
        "http://{}:{}/realms/{}/protocol/openid-connect/auth",
        args.keycloak_external_host, args.keycloak_external_port, args.keycloak_realm
    );
    log::info!("auth_uri: {}", auth_uri);

    // 3) TokenUrl: intern im Docker-Netzwerk
    let token_uri = format!(
        "http://{}:{}/realms/{}/protocol/openid-connect/token",
        args.keycloak_internal_host, args.keycloak_internal_port, args.keycloak_realm
    );
    log::info!("token_uri: {}", token_uri);

    // 4) Client-Secret laden
    let keycloak_secret = std::fs::read_to_string(&args.keycloak_secret_file)
        .expect("Cannot read Keycloak secret file");

    return BasicClient::new(ClientId::new(args.keycloak_client_id.clone()))
        .set_client_secret(ClientSecret::new(keycloak_secret))
        .set_auth_uri(AuthUrl::new(auth_uri).expect("Invalid external Auth URL"))
        .set_token_uri(TokenUrl::new(token_uri).expect("Invalid internal Token URL"))
        .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("Invalid redirect URI"));
}

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
        let session = req.get_session();
        let app = req
            .app_data::<Data<AppState>>()
            .cloned()
            .expect("AppState missing from request");

        // ✅ Session gültig → weiterleiten
        if let Ok(Some(_user_id)) = session.get::<Uuid>("user_id") {
            log::info!("Authenticated request from user_id: {}", _user_id);
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        log::info!("Redirecting user to keycload");

        let client = oauth_client(&app.args, &req.connection_info());
        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        // 🔥 Achtung: Jetzt erst `into_parts()` aufrufen
        let (req_head, _pl) = req.into_parts();

        Box::pin(async move {
            Ok(ServiceResponse::new(
                req_head,
                app.render_template(LoginView {
                    ctx: Context::new(&app, session),
                    login_url: auth_url.to_string(),
                }),
            ))
        })
    }
}
