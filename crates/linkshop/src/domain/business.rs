use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A business registered on the platform.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Business {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents an authenticated or resolved tenant in the application.
/// Queries that require business isolation must accept `&Tenant`
/// or derive scoping from it rather than accepting unverified identifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tenant {
    business: Business,
}

impl Tenant {
    pub fn new(business: Business) -> Self {
        Self { business }
    }

    pub fn business(&self) -> &Business {
        &self.business
    }

    pub fn id(&self) -> Uuid {
        self.business.id
    }

    pub fn slug(&self) -> &str {
        &self.business.slug
    }

    pub fn is_published(&self) -> bool {
        self.business.is_published
    }

    pub fn into_business(self) -> Business {
        self.business
    }
}
