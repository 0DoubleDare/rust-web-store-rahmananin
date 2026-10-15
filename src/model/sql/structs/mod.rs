use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct CategoryItem {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct AddCategoryForm {
    pub name: String,
}

#[derive(Deserialize)]
pub struct ProductItem {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub in_stock: i32,
    pub category: CategoryItem,
}

#[derive(Deserialize)]
pub struct AddProductForm {
    pub name: String,
    pub price: i64,
    pub in_stock: i64,
    pub category_id: i64,
}

#[derive(sqlx::FromRow)]
pub struct SqlProductItem {
    pub product_id: i32,
    pub product_name: String,
    pub price: i32,
    pub in_stock: i32,
    pub category_id: i32,
    pub category_name: String,
}
