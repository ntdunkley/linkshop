use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Slug(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SlugError {
    #[error("Slug '{0}' is a reserved system path and cannot be used")]
    Reserved(String),

    #[error("Slug '{0}' is invalid: must be non-empty lowercase alphanumeric and hyphens")]
    InvalidFormat(String),
}

impl Slug {
    pub const RESERVED: &'static [&'static str] = &["admin", "api", "login"];

    pub fn new(slug: impl Into<String>) -> Result<Self, SlugError> {
        let raw = slug.into().trim().to_lowercase();
        if Self::RESERVED.contains(&raw.as_str()) {
            return Err(SlugError::Reserved(raw));
        }

        if !Self::is_valid(&raw) {
            return Err(SlugError::InvalidFormat(raw));
        }

        Ok(Self(raw))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    fn is_valid(s: &str) -> bool {
        if s.is_empty() || s.len() > 100 {
            return false;
        }

        let mut prev_is_hyphen = true;
        for ch in s.chars() {
            if ch.is_ascii_lowercase() || ch.is_ascii_digit() {
                prev_is_hyphen = false;
            } else if ch == '-' {
                if prev_is_hyphen {
                    return false;
                }
                prev_is_hyphen = true;
            } else {
                return false;
            }
        }

        !prev_is_hyphen
    }
}

impl std::fmt::Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_slugs() {
        assert!(Slug::new("good-food").is_ok());
        assert!(Slug::new("bakery123").is_ok());
        assert!(Slug::new("my-cool-shop-2").is_ok());
    }

    #[test]
    fn rejects_reserved_slugs() {
        assert_eq!(
            Slug::new("admin"),
            Err(SlugError::Reserved("admin".to_string()))
        );
        assert_eq!(
            Slug::new("api"),
            Err(SlugError::Reserved("api".to_string()))
        );
        assert_eq!(
            Slug::new("login"),
            Err(SlugError::Reserved("login".to_string()))
        );
        assert_eq!(
            Slug::new("ADMIN"),
            Err(SlugError::Reserved("admin".to_string()))
        );
    }

    #[test]
    fn rejects_invalid_formats() {
        assert!(Slug::new("").is_err());
        assert!(Slug::new("-leading").is_err());
        assert!(Slug::new("trailing-").is_err());
        assert!(Slug::new("double--hyphen").is_err());
        assert!(Slug::new("has spaces").is_err());
        assert!(Slug::new("special!chars").is_err());
    }
}
