//! Database row types.
//!
//! These types mirror the SQL schema and exist only within the persistence
//! layer. Repositories translate them into domain entities so that `domain`
//! remains independent of the database representation.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::{Business, Item};

#[derive(Debug, Clone, sqlx::FromRow)]
pub(crate) struct BusinessRow {
    id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
    is_published: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<BusinessRow> for Business {
    fn from(row: BusinessRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            slug: row.slug,
            description: row.description,
            is_published: row.is_published,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub(crate) struct ItemRow {
    id: Uuid,
    business_id: Uuid,
    name: String,
    description: Option<String>,
    price: i64,
    is_available: bool,
    sort_order: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ItemRow> for Item {
    fn from(row: ItemRow) -> Self {
        Self {
            id: row.id,
            business_id: row.business_id,
            name: row.name,
            description: row.description,
            price: row.price,
            is_available: row.is_available,
            sort_order: row.sort_order,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
