use platform_core::ids::{OrganizationId, OrganizationMembershipId, UserId};
use serde::{Deserialize, Serialize};

/// Platform organization / tenant.
///
/// This is a generic organization container. It is not a church, shop, school,
/// branch, partner, or domain-specific profile. Domains attach their own
/// profiles and workflows to this tenant record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: OrganizationName,
    pub slug: OrganizationSlug,
    pub organization_type: OrganizationType,
    pub status: OrganizationStatus,
    pub country: CountryCode,
    pub currency: CurrencyCode,
    pub timezone: TimeZoneName,
    pub default_language: LanguageCode,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationMembership {
    pub id: OrganizationMembershipId,
    pub organization_id: OrganizationId,
    pub user_id: UserId,
    pub status: OrganizationMembershipStatus,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationName(String);

impl OrganizationName {
    pub fn new(value: impl Into<String>) -> Result<Self, OrganizationValidationError> {
        let value = value.into().trim().to_string();

        if value.len() < 2 || value.len() > 160 {
            return Err(OrganizationValidationError::InvalidName);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSlug(String);

impl OrganizationSlug {
    pub fn new(value: impl Into<String>) -> Result<Self, OrganizationValidationError> {
        let value = value.into().trim().to_lowercase();
        let is_valid = value.len() >= 3
            && value.len() <= 80
            && value
                .chars()
                .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-');

        if !is_valid || value.starts_with('-') || value.ends_with('-') {
            return Err(OrganizationValidationError::InvalidSlug);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CountryCode(String);

impl CountryCode {
    pub fn new(value: impl Into<String>) -> Result<Self, OrganizationValidationError> {
        let value = value.into().trim().to_uppercase();

        if value.len() != 2 || !value.chars().all(|character| character.is_ascii_uppercase()) {
            return Err(OrganizationValidationError::InvalidCountryCode);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyCode(String);

impl CurrencyCode {
    pub fn new(value: impl Into<String>) -> Result<Self, OrganizationValidationError> {
        let value = value.into().trim().to_uppercase();

        if value.len() != 3 || !value.chars().all(|character| character.is_ascii_uppercase()) {
            return Err(OrganizationValidationError::InvalidCurrencyCode);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneName(String);

impl TimeZoneName {
    pub fn new(value: impl Into<String>) -> Result<Self, OrganizationValidationError> {
        let value = value.into().trim().to_string();

        if value.len() < 3 || value.len() > 80 || !value.contains('/') {
            return Err(OrganizationValidationError::InvalidTimezone);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LanguageCode(String);

impl LanguageCode {
    pub fn new(value: impl Into<String>) -> Result<Self, OrganizationValidationError> {
        let value = value.into().trim().to_lowercase();

        if value.len() < 2 || value.len() > 12 {
            return Err(OrganizationValidationError::InvalidLanguageCode);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationType {
    Generic,
    Religious,
    Commerce,
    NonProfit,
    Education,
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationStatus {
    PendingSetup,
    Active,
    Suspended,
    Deactivated,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationMembershipStatus {
    Invited,
    Active,
    Suspended,
    Revoked,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum OrganizationValidationError {
    #[error("invalid organization name")]
    InvalidName,
    #[error("invalid organization slug")]
    InvalidSlug,
    #[error("invalid country code")]
    InvalidCountryCode,
    #[error("invalid currency code")]
    InvalidCurrencyCode,
    #[error("invalid timezone")]
    InvalidTimezone,
    #[error("invalid language code")]
    InvalidLanguageCode,
}

impl Organization {
    pub fn new_pending_setup(input: NewOrganization) -> Self {
        Self {
            id: OrganizationId::new(),
            name: input.name,
            slug: input.slug,
            organization_type: input.organization_type,
            status: OrganizationStatus::PendingSetup,
            country: input.country,
            currency: input.currency,
            timezone: input.timezone,
            default_language: input.default_language,
        }
    }
}

pub struct NewOrganization {
    pub name: OrganizationName,
    pub slug: OrganizationSlug,
    pub organization_type: OrganizationType,
    pub country: CountryCode,
    pub currency: CurrencyCode,
    pub timezone: TimeZoneName,
    pub default_language: LanguageCode,
}

impl OrganizationMembership {
    pub fn new_active_owner(organization_id: OrganizationId, user_id: UserId) -> Self {
        Self {
            id: OrganizationMembershipId::new(),
            organization_id,
            user_id,
            status: OrganizationMembershipStatus::Active,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_slug_shape() {
        assert!(OrganizationSlug::new("my-org-1").is_ok());
        assert!(OrganizationSlug::new("My Org").is_err());
        assert!(OrganizationSlug::new("-bad").is_err());
    }

    #[test]
    fn validates_country_and_currency_codes() {
        assert_eq!(CountryCode::new("gh").unwrap().as_str(), "GH");
        assert_eq!(CurrencyCode::new("ghs").unwrap().as_str(), "GHS");
        assert!(CountryCode::new("gha").is_err());
        assert!(CurrencyCode::new("gh").is_err());
    }
}
