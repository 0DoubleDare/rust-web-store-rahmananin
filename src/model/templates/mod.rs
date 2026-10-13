use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct HomePage<'a> {
    hello_message: &'a str,
}

pub async fn index_handler() -> HomePage<'static> {
    HomePage {
        hello_message: "Hello!",
    }
}
