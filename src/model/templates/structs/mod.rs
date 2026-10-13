use askama::Template;
use askama_web::WebTemplate;

use crate::model::{sql::structs::CategoryItem};

#[derive(Template, WebTemplate)]
#[template(path = "modules/categories/index.html")]
pub struct CategoriesPage {
    pub items: Vec<CategoryItem>,
}
