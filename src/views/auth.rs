use askama::Template;

use super::context::Context;

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login {
    pub ctx: Context,
}

#[derive(Template)]
#[template(path = "sign_up.html")]
pub struct SignUp {
    pub ctx: Context,
}
