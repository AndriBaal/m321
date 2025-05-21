use actix_web::{
    HttpResponse, HttpResponseBuilder,
    http::{StatusCode, header::HeaderValue},
};
use anyhow::Result;
use bson::{doc, oid::ObjectId};
use clap::{ArgGroup, Parser};
use log::info;
use mongodb::{Client, Database, options::ClientOptions};


/// CLI arguments for MongoDB connection
#[derive(Parser, Debug)]
#[command(name = "todo_app")]
#[command(about = "todo_app")]
#[command(group = ArgGroup::new("mongo_password_group")
    .args(&["mongo_password", "mongo_password_file"])
    .required(true))]
#[command(group = ArgGroup::new("session_secret_group")
    .args(&["session_secret", "session_secret_file"])
    .required(true))]
pub struct Args {
    // MongoDB
    #[arg(long, env, required = true)]
    mongo_username: String,
    #[arg(long, env)]
    mongo_password: Option<String>,
    #[arg(long, env)]
    mongo_password_file: Option<String>,
    #[arg(long, env, default_value = "todo_app")]
    mongo_database: Option<String>,
    #[arg(long, env, default_value = "localhost")]
    mongo_host: String,
    #[arg(long, env, default_value = "27017")]
    mongo_port: u16,

    // App
    #[arg(long, env, default_value = "false")]
    pub rebuild_indexes: bool,
    #[arg(long, env, default_value = "80")]
    pub port: u16,
    #[arg(long, env, value_parser = Self::validate_length)]
    pub session_secret: Option<String>,
    #[arg(long, env)]
    pub session_secret_file: Option<String>,
}

impl Args {
    pub fn validate_length(s: &str) -> Result<String, String> {
        if s.len() >= 64 {
            Ok(s.to_string())
        } else {
            Err("The session_secret must be at least 64 characters long.".to_string())
        }
    }
}


pub struct AppState {
    pub db: Database,
    pub args: Args
}

impl AppState {
    pub async fn new() -> Result<Self> {
        env_logger::init();

        let args = Args::parse();

        log::info!("{args:?}");

        let mongo_username = &args.mongo_username;
        let mongo_password = if let Some(mongo_password) = args.mongo_password.clone() {
            mongo_password
        } else if let Some(password_file) = &args.mongo_password_file {
            std::fs::read_to_string(password_file).unwrap()
        } else {
            unreachable!("No password provided! (Clap ensures one of the arguments is always set)")
        };
        let mongo_host = &args.mongo_host;
        let mongo_port = &args.mongo_port;
        let mut uri = format!(
            "mongodb://{}:{}@{}:{}",
            mongo_username, mongo_password, mongo_host, mongo_port
        );
        uri.retain(|c| !c.is_whitespace());

        // Create a MongoDB client
        let client_options = ClientOptions::parse(&uri).await?;
        let client = Client::with_options(client_options)?;

        let db = client.database(&args.mongo_database.clone().unwrap_or("".to_string()));

        match db.run_command(doc! {"ping": 1}).await {
            Ok(_) => {
                info!("MongoDB Authentication successful.");
            }
            Err(err) => {
                panic!("MongoDB Authentication failed: {}", err);
            }
        }

        Ok(Self {
            db,
            args,
        })
    }

    pub fn render_template<T: askama::Template>(&self, template: T) -> HttpResponse {
        let value = template.render().unwrap();
        HttpResponseBuilder::new(StatusCode::OK)
            .content_type(HeaderValue::from_static(T::MIME_TYPE))
            .body(value)
    }

    pub fn redirect(&self, location: &str) -> HttpResponse {
        HttpResponse::Found()
            .append_header(("Location", location))
            .finish()
    }

    pub fn try_user_id(&self, session: &actix_session::Session) -> Option<ObjectId> {
        session.get::<ObjectId>("user_id").unwrap()
    }

    pub fn user_id(&self, session: &actix_session::Session) -> ObjectId {
        self.try_user_id(session).unwrap()
    }
}
