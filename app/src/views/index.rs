use askama::Template;

use super::context::Context;
use crate::models::temperature_log::TemperatureLog;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexView {
    pub ctx: Context,
    pub avg_temperature: f32,
    pub avg_humidity: f32,
    pub entries: Vec<TemperatureLog>,
}
