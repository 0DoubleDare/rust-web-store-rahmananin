use askama::Template;
use askama_web::WebTemplate;

use crate::model::sql::structs::{CategoryItem, ProductItem};

pub struct BaseContext {
    pub page_title: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "modules/categories/index.html")]
pub struct CategoriesPage {
    pub base: BaseContext,
    pub items: Vec<CategoryItem>,
}

#[derive(Default)]
pub struct ProductPageFilter {
    pub date_from: chrono::NaiveDate,
    pub date_to: chrono::NaiveDate,
    pub categories_list: Vec<CategoryItem>,
    pub price_from: i64,
    pub price_to: i64,
    pub is_available: bool,
}

#[derive(Template, WebTemplate)]
#[template(path = "modules/products/index.html")]
pub struct ProductsPage {
    pub base: BaseContext,
    pub items: Vec<ProductItem>,
    pub filter: ProductPageFilter,
}
