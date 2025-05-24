use chrono::Duration;
use futures::TryStreamExt;

use crate::{
    app::AppState,
    controllers::auth::AuthRequired,
    views::{context::Context, index::IndexView},
};
use actix_session::Session;
use actix_web::{Responder, get, web::Data};
use bson::{DateTime, doc};

#[get("/", wrap = "AuthRequired")]
async fn index(app: Data<AppState>, session: Session) -> impl Responder {
    let now = DateTime::now().to_chrono();
    let diff = Duration::minutes(60);
    let ago = DateTime::from_chrono(now - diff);

    let pipeline = vec![
        doc! { "$match": { "time": { "$gte": ago } }},
        doc! { "$group": {
            "_id": null,
            "avg_temperature": { "$avg": "$temperature" },
            "avg_humidity": { "$avg": "$humidity" }
        }},
    ];

    let mut avg_temperature = 0.0;
    let mut avg_humidity = 0.0;
    let mut cursor = app.entries.aggregate(pipeline).await.unwrap();
    if let Some(result) = cursor.try_next().await.unwrap() {
        avg_temperature = result.get_f64("avg_temperature").unwrap() as f32;
        avg_humidity = result.get_f64("avg_humidity").unwrap() as f32;
    }

    let entries = app
        .entries
        .find(doc! {})
        .sort(doc! { "time": -1 })
        .limit(30)
        .await
        .unwrap()
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    app.render_template(IndexView {
        ctx: Context::new(&app, session),
        avg_temperature,
        avg_humidity,
        entries,
    })
}
