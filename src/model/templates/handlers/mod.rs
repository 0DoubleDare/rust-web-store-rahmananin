use axum::extract::{Query, State};

use crate::model::{
    server_queries::AppState,
    sql::{
        handlers::{get_categories, get_products},
        structs::{CategoryItem, ProductItem},
    },
    templates::structs::{BaseContext, CategoriesPage, ProductPageFilter, ProductsPage},
};

pub async fn category_page_handler(State(state): State<AppState>) -> CategoriesPage {
    let categories: Vec<CategoryItem> = get_categories(&state).await.unwrap_or_default();

    CategoriesPage {
        base: BaseContext {
            page_title: String::from("Категории"),
        },
        items: categories,
    }
}

pub async fn products_page_handler(State(state): State<AppState>, Query(filter): Query<ProductPageFilter>) -> ProductsPage {
    let categories: Vec<CategoryItem> = get_categories(&state).await.unwrap_or_default();
    let products: Vec<ProductItem> = get_products(&state, &filter).await;
    ProductsPage {
        base: BaseContext {
            page_title: String::from("Продукты"),
        },
        items: products,
        filter: filter,
        categories: categories,
    }
}
