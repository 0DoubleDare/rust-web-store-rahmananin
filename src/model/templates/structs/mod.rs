use askama::Template;
use askama_web::WebTemplate;
use serde::Deserialize;
use serde_with::serde_as;

use crate::model::sql::structs::{CategoryItem, ProductItem};

#[derive(Default)]
pub struct BaseContext {
    pub page_title: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "modules/categories/index.html")]
pub struct CategoriesPage {
    pub base: BaseContext,
    pub items: Vec<CategoryItem>,
}

#[derive(Default, Deserialize)]
pub struct ProductPageFilter {
    pub category_id: Option<i64>,
    pub price_from: Option<i64>,
    pub price_to: Option<i64>,
    pub in_stock: Option<i64>,
}

#[derive(Template, WebTemplate, Default)]
#[template(path = "modules/products/index.html")]
pub struct ProductsPage {
    pub base: BaseContext,
    pub items: Vec<ProductItem>,
    pub categories: Vec<CategoryItem>,
    pub filter: ProductPageFilter,
}
