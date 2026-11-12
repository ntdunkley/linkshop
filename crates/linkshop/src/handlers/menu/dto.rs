use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{Business, Item};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BusinessProfile {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

impl From<Business> for BusinessProfile {
    fn from(business: Business) -> Self {
        Self {
            id: business.id,
            name: business.name,
            slug: business.slug,
            description: business.description,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MenuResponse {
    pub business: BusinessProfile,
    pub items: Vec<MenuItem>,
}
