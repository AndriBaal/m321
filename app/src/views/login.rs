use askama::Template;

use super::context::Context;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginView {
    pub ctx: Context,
    pub login_url: String,
}
