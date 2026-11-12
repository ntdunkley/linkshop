use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{BusinessProfile, MenuItem, Slug, Tenant},
    error::AppError,
    repos::{BusinessRepository, ItemRepository, PgBusinessRepository, PgItemRepository},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MenuResponse {
    pub business: BusinessProfile,
    pub items: Vec<MenuItem>,
}

pub async fn get_menu(
    path: web::Path<String>,
    business_repo: web::Data<PgBusinessRepository>,
    item_repo: web::Data<PgItemRepository>,
) -> Result<HttpResponse, AppError> {
    let slug_raw = path.into_inner();

    let slug = Slug::new(&slug_raw)
        .map_err(|_| AppError::NotFound(format!("Business '{slug_raw}' not found")))?;

    let business = business_repo
        .find_published_by_slug(slug.as_str())
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Business '{}' not found", slug.as_str())))?;

    let tenant = Tenant::new(business);

    let items = item_repo.list_by_business_id(tenant.id()).await?;

    let response = MenuResponse {
        business: BusinessProfile::from(tenant.into_business()),
        items: items.into_iter().map(MenuItem::from).collect(),
    };

    Ok(HttpResponse::Ok().json(response))
}
