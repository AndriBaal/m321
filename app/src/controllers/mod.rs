use crate::app::{AppState, Args};

use actix_files::Files;
use actix_session::{SessionMiddleware, config::PersistentSession};
use actix_web::{
    App, HttpServer,
    cookie::{Key, time::Duration},
    middleware::Logger,
    web,
};

pub mod auth;
pub mod index;

pub async fn setup_server(app: AppState) -> std::io::Result<()> {
    let mongo_session_store: crate::models::session::SessionManager =
        crate::models::session::connect(&app.db).await;
    let session_secret = if let Some(session_secret) = app.args.session_secret.clone() {
        session_secret
    } else if let Some(password_file) = &app.args.session_secret_file {
        std::fs::read_to_string(password_file).unwrap()
    } else {
        unreachable!(
            "No session secret provided! (Clap ensures one of the arguments is always set)"
        )
    };
    Args::validate_length(&session_secret).unwrap();
    let bytes = session_secret.as_bytes();
    let secret_key = Key::from(bytes);
    let port = app.args.port;
    let app_data = web::Data::new(app);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(mongo_session_store.clone(), secret_key.clone())
                    .cookie_http_only(true)
                    .cookie_secure(false)
                    .cookie_same_site(actix_web::cookie::SameSite::Strict)
                    .cookie_content_security(actix_session::config::CookieContentSecurity::Private)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::weeks(2))
                            .session_ttl_extension_policy(
                                actix_session::config::TtlExtensionPolicy::OnEveryRequest,
                            ),
                    )
                    .build(),
            )
            .service(Files::new("/static", "./static").show_files_listing())
            .service(index::index)
            .service(auth::login)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
