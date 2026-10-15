use axum::{
    Form,
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};
use sqlx::{MySql, QueryBuilder, mysql::MySqlQueryResult};

use crate::model::{
    server_queries::AppState,
    sql::structs::{AddCategoryForm, AddProductForm, CategoryItem, ProductItem, SqlProductItem},
    templates::structs::ProductPageFilter,
};

pub async fn get_categories(state: &AppState) -> Result<Vec<CategoryItem>, sqlx::Error> {
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

pub async fn add_product(
    State(state): State<AppState>,
    Form(input): Form<AddProductForm>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO products(name, price, in_stock, category_id) VALUES (?, ?, ?, ?)",
        input.name,
        input.price,
        input.in_stock,
        input.category_id,
    )
    .execute(&state.db)
    .await;

    Redirect::to("/products")
}

pub async fn get_products(state: &AppState, filter: &ProductPageFilter) -> Vec<ProductItem> {
    let mut products: Vec<ProductItem> = Vec::new();

    let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
        "SELECT
            p.id AS product_id,
            p.name AS product_name,
            p.price,
            p.in_stock,
            c.name AS category_name,
            c.id AS category_id
        FROM products p
                 INNER JOIN categories c ON p.category_id = c.id
        WHERE
            1=1
            ",
    );

    if let Some(price_to) = filter.price_from {
        query_builder.push("AND p.price >=").push_bind(price_to);
    }
    if let Some(price_from) = filter.price_to {
        query_builder.push("AND p.price <=").push_bind(price_from);
    }
    if let Some(category) = filter.category_id {
        query_builder
            .push("AND p.category_id =")
            .push_bind(category);
    }
    if let Some(in_stock) = filter.in_stock {
        query_builder.push("AND p.in_stock >=").push_bind(in_stock);
    }

    let result = query_builder
        .build_query_as::<SqlProductItem>()
        .fetch_all(&state.db)
        .await
        .unwrap();

    for product in result {
        products.push(ProductItem {
            id: product.product_id,
            name: product.product_name,
            price: product.price,
            in_stock: product.in_stock,
            category: CategoryItem {
                id: product.category_id,
                name: product.category_name,
            },
        });
    }

    products
}
