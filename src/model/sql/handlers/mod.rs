
use axum::{
    Form,
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};

use crate::model::{
    server_queries::AppState,
    sql::structs::{AddCategoryForm, CategoryItem},
};

pub async fn get_categories(
    State(state): State<AppState>,
) -> Result<Vec<CategoryItem>, sqlx::Error> {
    let categories: Vec<CategoryItem> =
        sqlx::query_as!(CategoryItem, "SELECT id, name FROM categories")
            .fetch_all(&state.db)
            .await?;

    Ok(categories)
}

pub async fn add_category(
    State(state): State<AppState>,
    Form(input): Form<AddCategoryForm>,
) -> impl IntoResponse {
    let result = sqlx::query!("INSERT INTO categories(name) VALUES (?)", input.name)
        .execute(&state.db)
        .await;

    tracing::info!("{:#?}", result);

    Redirect::to("/categories")
}

pub async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let product_count_by_category = count_products_by_category(&state, id).await;
    match product_count_by_category {
        Ok(val) => {
            if val > 0 {
                tracing::info!("Нельзя удалить категорию так как у неё есть товары");
                return Redirect::to("/categories");
            }
        }
        Err(err) => {
            tracing::error!("Ошибка при запросе {:#?}", err);
        }
    }

    let _ = sqlx::query!("DELETE FROM categories WHERE id = ?", id)
        .execute(&state.db)
        .await;

    Redirect::to("/categories")
}

pub async fn count_products_by_category(state: &AppState, id: i32) -> Result<i64, sqlx::Error> {
    let result: i64 = sqlx::query_scalar!(
        "SELECT
            COUNT(*) as count
        FROM `products`
        WHERE category_id = ?
        GROUP BY category_id",
        id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(result)
}
