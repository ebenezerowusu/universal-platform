use crate::models::{
    CountryCode, CurrencyCode, LanguageCode, NewOrganization, Organization, OrganizationMembership,
    OrganizationName, OrganizationSlug, OrganizationType, TimeZoneName,
};
use crate::ports::OrganizationRepository;
use platform_core::{ids::UserId, PlatformError, PlatformResult};

pub struct OrganizationService<R>
where
    R: OrganizationRepository,
{
    repository: R,
}

impl<R> OrganizationService<R>
where
    R: OrganizationRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_pending_organization(
        &self,
        input: CreateOrganizationInput,
        owner_user_id: UserId,
    ) -> PlatformResult<Organization> {
        let slug = OrganizationSlug::new(input.slug)
            .map_err(|error| PlatformError::Validation(error.to_string()))?;

        if self.repository.find_by_slug(&slug).await?.is_some() {
            return Err(PlatformError::Validation("organization slug is already in use".into()));
        }

        let organization = Organization::new_pending_setup(NewOrganization {
            name: OrganizationName::new(input.name)
                .map_err(|error| PlatformError::Validation(error.to_string()))?,
            slug,
            organization_type: input.organization_type,
            country: CountryCode::new(input.country)
                .map_err(|error| PlatformError::Validation(error.to_string()))?,
            currency: CurrencyCode::new(input.currency)
                .map_err(|error| PlatformError::Validation(error.to_string()))?,
            timezone: TimeZoneName::new(input.timezone)
                .map_err(|error| PlatformError::Validation(error.to_string()))?,
            default_language: LanguageCode::new(input.default_language)
                .map_err(|error| PlatformError::Validation(error.to_string()))?,
        });

        let membership = OrganizationMembership::new_active_owner(organization.id.clone(), owner_user_id);

        self.repository.save_organization(&organization).await?;
        self.repository.save_membership(&membership).await?;

        Ok(organization)
    }
}

pub struct CreateOrganizationInput {
    pub name: String,
    pub slug: String,
    pub organization_type: OrganizationType,
    pub country: String,
    pub currency: String,
    pub timezone: String,
    pub default_language: String,
}
