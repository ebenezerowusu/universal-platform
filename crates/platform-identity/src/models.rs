use platform_core::ids::UserId;
use serde::{Deserialize, Serialize};

/// Platform-wide identity user.
///
/// This is not a member, customer, staff profile, or domain actor.
/// Domain-specific profiles link to this identity user when needed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IdentityUser {
    pub id: UserId,
    pub email: EmailAddress,
    pub password_hash: PasswordHash,
    pub status: IdentityUserStatus,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(value: impl Into<String>) -> Result<Self, EmailValidationError> {
        let value = value.into().trim().to_lowercase();

        if value.is_empty() || !value.contains('@') || value.len() > 320 {
            return Err(EmailValidationError);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmailValidationError;

impl std::fmt::Display for EmailValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "invalid email address")
    }
}

impl std::error::Error for EmailValidationError {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn from_hash(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityUserStatus {
    PendingVerification,
    Active,
    Suspended,
    Deactivated,
}

impl IdentityUser {
    pub fn new_pending(email: EmailAddress, password_hash: PasswordHash) -> Self {
        Self {
            id: UserId::new(),
            email,
            password_hash,
            status: IdentityUserStatus::PendingVerification,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_is_normalized() {
        let email = EmailAddress::new("  USER@Example.COM  ").expect("email should be valid");

        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn invalid_email_is_rejected() {
        let result = EmailAddress::new("not-an-email");

        assert!(result.is_err());
    }

    #[test]
    fn new_identity_user_starts_pending_verification() {
        let email = EmailAddress::new("user@example.com").expect("email should be valid");
        let hash = PasswordHash::from_hash("hashed-password-placeholder");

        let user = IdentityUser::new_pending(email, hash);

        assert_eq!(user.status, IdentityUserStatus::PendingVerification);
    }
}
