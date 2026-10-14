use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct CategoryItem {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct AddCategoryForm {
    pub name: String,
}

#[derive(Deserialize)]
pub struct ProductItem {
    pub id: i64,
    pub name: String,
    pub price: i64,
    pub in_stock: i64,
    pub cateogory: CategoryItem,
}
