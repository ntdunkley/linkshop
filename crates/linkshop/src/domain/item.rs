use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, PartialEq, Eq)]
pub struct Item {
    pub id: Uuid,
    pub business_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub is_available: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MenuItem {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub is_available: bool,
}

impl From<Item> for MenuItem {
    fn from(item: Item) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            price: item.price,
            is_available: item.is_available,
        }
    }
}
