use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CategoryItem {
    pub id: i64,
    pub name: String,
}
