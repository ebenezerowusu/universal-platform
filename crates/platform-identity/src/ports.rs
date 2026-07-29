use crate::models::{EmailAddress, IdentityUser, PasswordHash};
use platform_core::{errors::PlatformResult, ids::UserId};

/// Persistence boundary for identity users.
///
/// Implementations belong in infrastructure crates, not in this domain crate.
pub trait IdentityRepository {
    fn find_by_id(&self, id: &UserId) -> impl std::future::Future<Output = PlatformResult<Option<IdentityUser>>> + Send;

    fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> impl std::future::Future<Output = PlatformResult<Option<IdentityUser>>> + Send;

    fn create_user(&self, user: IdentityUser) -> impl std::future::Future<Output = PlatformResult<IdentityUser>> + Send;
}

/// Password hashing boundary.
///
/// Real hashing implementations must use a reviewed algorithm such as Argon2.
/// Plain-text password handling must never cross persistence boundaries.
pub trait PasswordHasher {
    fn hash_password(&self, plain_password: &str) -> PlatformResult<PasswordHash>;

    fn verify_password(&self, plain_password: &str, password_hash: &PasswordHash) -> PlatformResult<bool>;
}
