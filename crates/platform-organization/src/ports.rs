use crate::models::{Organization, OrganizationMembership, OrganizationSlug};
use platform_core::{ids::{OrganizationId, UserId}, PlatformResult};

/// Repository boundary for organization and membership persistence.
///
/// Implementations must preserve tenant and membership isolation. This trait
/// intentionally exposes generic platform organization concepts only.
pub trait OrganizationRepository: Send + Sync {
    fn find_by_id(&self, organization_id: &OrganizationId) -> impl std::future::Future<Output = PlatformResult<Option<Organization>>> + Send;

    fn find_by_slug(&self, slug: &OrganizationSlug) -> impl std::future::Future<Output = PlatformResult<Option<Organization>>> + Send;

    fn save_organization(&self, organization: &Organization) -> impl std::future::Future<Output = PlatformResult<()>> + Send;

    fn save_membership(&self, membership: &OrganizationMembership) -> impl std::future::Future<Output = PlatformResult<()>> + Send;

    fn list_user_memberships(&self, user_id: &UserId) -> impl std::future::Future<Output = PlatformResult<Vec<OrganizationMembership>>> + Send;
}
