use askama::Template;
use askama_web::WebTemplate;

use crate::model::sql::structs::CategoryItem;

pub struct BaseContext {
    pub page_title: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "modules/categories/index.html")]
pub struct CategoriesPage {
    pub base: BaseContext,
    pub items: Vec<CategoryItem>,
}
