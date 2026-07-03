# Entity And Data Model Catalogue

## Purpose

This document consolidates the conceptual entities across Universal Platform foundations.

It is not a final database schema. Exact columns, indexes, constraints, and migrations should be created during implementation.

## Global Data Model Rules

Every organization-owned record should include:

```text
id
organization_id
status
created_at
updated_at
created_by
updated_by
```

Sensitive or operational records should also include:

```text
audit_context
source_system
metadata
review_status
```

Use UUIDs or stable IDs for public identifiers.

## Core Entities

```text
User
UserProfile
Organization
OrganizationMembership
OrganizationInvitation
OrganizationSetting
OrganizationLifecycleState
TenantProvisioningRecord
Session
Device
AccessLifecycleRecord
```

## Permission And Audit Entities

```text
Permission
Role
RolePermission
UserRoleAssignment
PermissionPolicy
AuditEvent
SensitiveActionRecord
AuditMetadata
```

## Module, Subscription, And Configuration Entities

```text
ModuleDefinition
ModuleInstallation
ModuleEntitlement
SubscriptionPlan
SubscriptionRecord
BillingRecord
InvoicePlaceholder
ConfigurationNamespace
ConfigurationKey
ConfigurationValue
TenantConfigurationOverride
FeatureFlag
FeatureFlagEvaluationLog
RuntimeRule
```

## Provider, Payment, SMS, And Notification Entities

```text
ProviderConnection
ProviderCredentialReference
ProviderCapabilityMapping
PaymentTransaction
PaymentProviderOperation
PaymentReconciliationRecord
SmsWallet
SmsPackage
SmsCreditLedger
SmsMessage
CommunicationTemplate
CommunicationPolicy
Notification
NotificationPreference
NotificationDeliveryRecord
```

## Religious Domain Entities

```text
ReligiousMember
Visitor
Household
HouseholdRelationship
Congregation
Service
Group
Department
Cell
BibleStudyProgram
BibleStudyGroup
AttendanceSession
AttendanceRecord
GivingRecord
ReligiousReportPlaceholder
```

## Finance Entities

```text
IncomeCategory
IncomeRecord
GivingCategory
ExpenseCategory
ExpenseRecord
Budget
BudgetLine
CashPositionSummary
FinanceReport
```

## Commerce, Marketplace, Fulfillment, And Logistics Entities

```text
Product
ProductCategory
InventoryItem
InventoryMovement
PosSale
MarketplaceListing
CartPlaceholder
Order
OrderItem
OrderStatusHistory
FulfillmentTask
PickupRecord
DeliveryRecord
DeliveryAssignment
LogisticsStatusHistory
```

## HR, Scheduling, Documents, Search, And Reporting Entities

```text
StaffProfile
StaffAttendanceSession
StaffAttendanceRecord
LeaveRequest
CalendarEvent
Schedule
Booking
DocumentFile
MediaItem
Folder
SearchIndexRecord
ReportDefinition
DashboardDefinition
ReportRun
ExportRequest
```

## Workflow, Jobs, Import, Export, And Migration Entities

```text
WorkflowDefinition
WorkflowInstance
WorkflowStep
WorkflowTask
ApprovalRecord
BackgroundJob
QueueRecord
ScheduledTask
JobRun
ImportJob
ImportMapping
ImportValidationResult
ExportJob
MigrationRun
MigrationValidationResult
```

## Data Governance Entities

```text
DataClassification
SensitiveFieldCatalogItem
ConsentPurpose
ConsentRecord
PrivacyAccessLog
MaskingPolicy
SubjectRequestPlaceholder
DataQualityRule
DataQualityValidationResult
DuplicateCandidate
MergeReview
CanonicalRecordMarker
DataCorrectionRequest
SourceSystemMetadata
RecordProvenance
RecordVersionHistory
LineageLink
PortabilityExportPackage
TenantExitReview
HandoverManifest
```

## Organization Network And Governance Entities

```text
OrganizationRelationship
BranchConnectionRequest
BranchDisconnectRequest
HierarchyPath
DataSharingPolicy
HistoricalAccessPolicy
GovernancePolicy
AuthorityMandate
DelegatedApproverAssignment
ApprovalThresholdRule
AuthorityDecisionLog
```

## Risk, Compliance, Policy, And Knowledge Entities

```text
RiskRegisterItem
RiskAssessment
ControlObjective
ControlCheck
EvidenceItem
ControlException
RemediationAction
ComplianceChecklist
PolicyDocument
PolicyVersion
PolicyPublication
AttestationRequest
AcknowledgementRecord
TrainingAssignment
HelpArticle
HelpCategory
HelpArticleFeedback
```

## Support, Product Communication, Feedback, Analytics, And Experimentation Entities

```text
SupportTicket
SupportEscalation
SupportStatusHistory
ProductAnnouncement
ReleaseNote
ChangeNotice
MaintenanceNotice
AnnouncementReadReceipt
FeedbackItem
FeatureRequest
RoadmapCandidate
FeedbackStatusUpdate
UsageEventDefinition
FeatureAdoptionSummary
ModuleEngagementSummary
OnboardingFunnelSummary
ActivationRetentionSummary
AnalyticsAggregationRule
Experiment
ExperimentVariant
RolloutCohort
SuccessMetricLink
GuardrailMetricLink
ExperimentDecisionReview
```

## Customer Success, Implementation, And Blueprints Entities

```text
AccountHealthSummary
AdoptionHealthSignal
RiskSignal
SuccessPlaybook
PlaybookTask
CustomerIntervention
SuccessNote
ImplementationProject
OnboardingMilestone
MigrationRunbook
SetupTask
GoLiveReadinessReview
ImplementationIssueRisk
ImplementationNote
SolutionBlueprint
BlueprintVersion
ConfigurationTemplate
DeploymentPackage
ComponentManifest
PreflightCheck
PreviewApplyPlan
ChangeSummary
```

## Partner Ecosystem Entities

```text
PartnerOrganization
ImplementerProfile
PartnerCertification
PartnerBadge
PartnerAssignment
DelegationRequest
DelegatedAccessScope
ImplementationBlueprintLink
PartnerQualityReview
EnablementProgram
TrainingPath
TrainingContentLink
CertificationProgram
AssessmentCheckpoint
CertificationAttempt
CertificationRenewal
ReferralLead
OpportunityRegistration
CoSellPipelineStage
EligibilityReview
ConflictReview
PartnerStatusUpdate
```

## Developer, Operator, And Commercial Entities

```text
DeveloperApiClient
WebhookSubscription
WebhookDelivery
ModuleSubmission
ModuleReview
ExtensionCompatibilityCheck
OperatorAction
OperatorDashboardSummary
CommercialPackage
PlanAddOn
ServiceCatalogueItem
UsageRevenueSignal
```

## Entity Ownership Rules

- Identity entities are global but access is scoped by organization membership.
- Organization entities define tenant boundaries.
- Domain entities must never skip `organization_id`.
- Provider credentials must be references, not raw secrets.
- Audit entities must be append-oriented.
- Data lineage and history entities should preserve provenance.
- Partner entities must separate partner organization from customer organization.

## Final Rule

This catalogue guides implementation, but migrations must still be reviewed for tenancy, indexes, constraints, and audit requirements.
