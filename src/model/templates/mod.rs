pub mod handlers;
pub mod structs;


use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;

use crate::model::server_queries::AppState;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct HomePage<'a> {
    hello_message: &'a str,
}

pub async fn index_handler(State(state): State<AppState>) -> HomePage<'static> {
    HomePage {
        hello_message: "Hello!",
    }
}
