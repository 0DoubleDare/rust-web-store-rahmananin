use axum::extract::State;

use crate::model::{
    server_queries::AppState,
    sql::{handlers::get_categories, structs::CategoryItem},
    templates::structs::{BaseContext, CategoriesPage},
};

pub async fn category_page_handler(State(state): State<AppState>) -> CategoriesPage {
    let categories: Vec<CategoryItem> = get_categories(State(state)).await.unwrap_or_default();

    CategoriesPage {
        base: BaseContext {
            page_title: String::from("Категории"),
        },
        items: categories,
    }
}
