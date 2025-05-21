use askama::Template;

use super::context::Context;


#[derive(Template)]
#[template(path = "device.html")]
pub struct DeviceView {
    pub ctx: Context,
}
