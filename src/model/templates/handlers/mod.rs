use axum::extract::State;

use crate::model::{
    server_queries::AppState,
    sql::{handlers::get_categories, structs::CategoryItem},
    templates::structs::CategoriesPage,
};

pub async fn category_page_handler(State(state): State<AppState>) -> CategoriesPage {
    let categories: Vec<CategoryItem> = get_categories(State(state)).await.unwrap_or_default();

    CategoriesPage { items: categories }
}
