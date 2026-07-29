use crate::{
    models::{EmailAddress, IdentityUser},
    ports::{IdentityRepository, PasswordHasher},
};
use platform_core::{errors::PlatformResult, ids::UserId};

#[derive(Clone, Debug)]
pub struct CreateUserInput {
    pub email: String,
    pub plain_password: String,
}

pub struct IdentityService<R, H>
where
    R: IdentityRepository,
    H: PasswordHasher,
{
    repository: R,
    password_hasher: H,
}

impl<R, H> IdentityService<R, H>
where
    R: IdentityRepository,
    H: PasswordHasher,
{
    pub fn new(repository: R, password_hasher: H) -> Self {
        Self {
            repository,
            password_hasher,
        }
    }

    pub async fn create_user(&self, input: CreateUserInput) -> PlatformResult<IdentityUser> {
        let email = EmailAddress::new(input.email).map_err(|error| {
            platform_core::errors::PlatformError::Validation(error.to_string())
        })?;

        if self.repository.find_by_email(&email).await?.is_some() {
            return Err(platform_core::errors::PlatformError::Conflict(
                "identity user email already exists".to_string(),
            ));
        }

        let password_hash = self.password_hasher.hash_password(&input.plain_password)?;
        let user = IdentityUser::new_pending(email, password_hash);

        self.repository.create_user(user).await
    }

    pub async fn get_user(&self, id: &UserId) -> PlatformResult<Option<IdentityUser>> {
        self.repository.find_by_id(id).await
    }
}
