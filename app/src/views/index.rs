use askama::Template;

use super::context::Context;


#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexView {
    pub ctx: Context,
}
