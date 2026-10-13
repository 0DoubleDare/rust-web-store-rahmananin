use axum::extract::State;

use crate::model::{server_queries::AppState, sql::structs::CategoryItem};

pub async fn get_categories(
    State(state): State<AppState>,
) -> Result<Vec<CategoryItem>, sqlx::Error> {
    let categories: Vec<CategoryItem> =
        sqlx::query_as!(CategoryItem, "SELECT id, name FROM categories")
            .fetch_all(&state.db)
            .await?;

    Ok(categories)
}
