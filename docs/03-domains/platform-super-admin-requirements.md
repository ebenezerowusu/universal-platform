# Platform Super Admin Requirements

## Purpose

This document defines the required features and operating boundaries for the Platform Super Admin area.

The Platform Super Admin is the internal Hulnex/platform operator role. It is not the same as an organization admin, church admin, branch admin, pastor, finance officer, or group leader.

The Super Admin controls the SaaS platform, but organization-owned data still belongs to each organization.

## Position In The Platform

```text
Platform Super Admin
  -> manages the whole SaaS platform

Organization Admin
  -> manages one organization/church/institution account

Branch / Department / Group Admin
  -> manages only assigned scope inside an organization
```

The Super Admin area belongs to the platform operator control plane.

It must remain domain-agnostic. It should manage organizations, modules, plans, providers, support, security, operations, and billing across the platform without becoming church-specific.

## Core Rules

```text
Super Admin controls the platform.
Organization data belongs to the organization.
Super Admin access to organization data must be permission-controlled, reason-based, time-limited where needed, and fully audited.
```

The Super Admin must not have unrestricted silent access to private organization records.

Forbidden behavior:

- unrestricted customer data browsing
- hidden tenant access
- bypassing audit logs
- bypassing permission checks
- destructive actions without review
- shared Super Admin accounts
- direct provider secrets exposed to normal admins
- cross-tenant data visibility leaks

## 1. Platform Dashboard

The Super Admin dashboard gives a high-level view of platform health, usage, and revenue.

Required dashboard metrics:

- total organizations
- active organizations
- trial organizations
- suspended organizations
- deactivated organizations
- total platform users
- total organization admins
- total end users/members across organizations
- active subscriptions
- failed renewals
- monthly recurring revenue placeholder
- communication credits sold
- SMS usage
- Email usage
- WhatsApp usage
- payment volume
- failed payment count
- system incidents
- support tickets
- provider health summary

Example dashboard cards:

```text
Total Organizations
Active Subscriptions
Trial Organizations
Monthly Revenue
SMS Credits Sold
Email Credits Sold
WhatsApp Credits Sold
Payment Volume
Failed Payments
Open Support Tickets
Platform Incidents
```

## 2. Organization / Tenant Management

Super Admin can manage organizations as tenants.

Required features:

- create organization
- view organization profile
- approve organization registration
- update organization administrative metadata
- suspend organization
- reactivate organization
- deactivate organization
- view organization country, currency, timezone, and language settings
- view organization subscription summary
- view organization enabled modules
- view organization usage summary
- view branch count summary
- view storage usage summary
- view communication wallet balances
- view payment transaction summary
- view organization audit summary

Organization statuses:

```text
pending_review
active
trial
suspended
deactivated
closed
```

Suspension rules:

- suspended organizations cannot access paid functionality unless explicitly allowed
- suspension must record reason, actor, timestamp, and affected modules
- reactivation must be audited
- deactivation must require stronger confirmation than suspension

## 3. Support Access Management

Support access allows platform staff to help an organization without giving unrestricted hidden access.

Required features:

- request support access
- approve support access
- reject support access
- expire support access automatically
- revoke support access manually
- require reason for access
- scope access to organization, module, and time window
- audit every support access action
- show active support sessions
- show historical support sessions

Support access statuses:

```text
requested
approved
rejected
active
expired
revoked
```

Support access rules:

- support access must be time-limited
- sensitive finance/member data should require elevated permission
- support agents should not be able to silently access private data
- all actions performed during support access must be tagged with support_access_id

## 4. Subscription And Plan Management

Super Admin manages platform plans and organization subscriptions.

Required features:

- create plan
- update plan
- archive plan
- set plan limits
- assign plan to organization
- upgrade organization plan
- downgrade organization plan
- cancel subscription
- extend trial
- apply discount placeholder
- view invoices
- view failed renewals
- view payment history
- view subscription status
- view revenue summary

Example plans:

```text
Free Trial
Starter
Growth
Professional
Enterprise
Custom
```

Plan limits may include:

- maximum users
- maximum members/end users
- maximum branches
- maximum admins
- enabled modules
- storage limit
- API access
- communication package eligibility
- report access
- support level

## 5. Module Management

Super Admin controls platform modules and organization module access.

Required features:

- create module metadata
- update module metadata
- enable module globally
- disable module globally
- enable module for organization
- disable module for organization
- set module dependencies
- set module plan availability
- set module pricing placeholder
- view module usage
- view module rollout status
- view organizations using module

Example modules:

```text
Religious Management
Attendance
Giving And Payments
Communication
Bible Study
Welfare
Events
Finance
HR
POS
Marketplace
Logistics
```

Activation rule:

```text
A module is active for an organization only when:
1. the module is globally active
2. the organization has the module enabled
3. the subscription plan allows the module
4. the organization is not blocked from using it
```

## 6. Communication Package Management

Super Admin manages sellable SMS, Email, WhatsApp, and multi-channel bundles.

Required features:

- create SMS package
- create Email package
- create WhatsApp package
- create multi-channel bundle
- update package price
- update package credit quantity
- activate/deactivate package
- assign free credits
- top up organization wallet
- deduct credits manually with reason
- view credit purchases
- view usage history
- view delivery failures
- view provider cost placeholder
- view profit margin placeholder

Example packages:

```text
SMS 1,000 Credits
Email 5,000 Credits
WhatsApp 500 Credits
Visitor Follow-Up Bundle
Pledge Reminder Bundle
Announcement Bundle
```

Rule:

```text
Organizations see simple wallet balances and usage.
Super Admin sees package pricing, provider cost, margin, delivery health, and provider routing.
```

## 7. Payment Provider Management

Super Admin manages payment provider configuration by country and channel.

Required features:

- configure payment provider metadata
- enable provider by country
- disable provider by country
- set default provider
- set fallback provider
- view provider transaction summaries
- view provider failure summaries
- view settlement/reconciliation status placeholder
- rotate provider credential references
- disable provider during incident

Provider examples:

```text
Ghana -> Hubtel / Paystack / Flutterwave
Nigeria -> Paystack / Flutterwave
Kenya -> M-Pesa
South Africa -> PayFast / Ozow
```

Rules:

- provider secrets must never be shown in plain text
- organization admins must not manage low-level provider secrets
- failed provider callbacks must be visible to platform operators
- provider changes must be audited

## 8. SMS, Email, And WhatsApp Provider Management

Super Admin manages communication providers behind the Communication Engine.

Required features:

- configure SMS provider metadata
- configure Email provider metadata
- configure WhatsApp provider metadata
- enable provider by country
- disable provider by country
- set fallback provider
- view delivery rate
- view failed delivery count
- view provider downtime
- view provider balance placeholder
- view provider cost placeholder
- reroute traffic during incidents

Rules:

- organizations do not see provider secrets
- organizations do not need to understand provider routing
- organizations see message delivery status, not provider internals
- provider changes must be audited

## 9. Country, Currency, And Localization Management

Super Admin manages platform regional settings.

Required features:

- manage countries
- manage currencies
- manage timezone defaults
- manage language availability
- manage date/time formats
- manage phone number rules
- manage country-specific payment options
- manage country-specific communication providers
- manage default regional settings for new organizations

Initial language direction:

```text
English
Twi
French
Swahili
```

Future languages can be added without rewriting domain logic.

## 10. Platform Staff And Access Management

Super Admin manages internal platform staff accounts.

Required features:

- create platform staff user
- update platform staff user
- suspend platform staff user
- reactivate platform staff user
- assign platform role
- revoke platform role
- revoke sessions
- enforce MFA for sensitive roles
- view platform staff audit history

Platform roles:

```text
Owner
Super Admin
Support Admin
Billing Admin
Technical Admin
Compliance Admin
Read-only Auditor
```

Rules:

- no shared admin accounts
- every action must map to a named actor
- MFA should be required for high-risk roles
- dangerous permission assignment must be audited

## 11. Platform Permission Management

Super Admin manages platform-level permission definitions and role assignment.

Example permissions:

```text
platform.dashboard.view
platform.organizations.view
platform.organizations.manage
platform.organizations.suspend
platform.organizations.reactivate
platform.subscriptions.view
platform.subscriptions.manage
platform.modules.view
platform.modules.manage
platform.communication_packages.view
platform.communication_packages.manage
platform.payment_providers.view
platform.payment_providers.manage
platform.communication_providers.view
platform.communication_providers.manage
platform.support_access.request
platform.support_access.approve
platform.support_access.revoke
platform.audit.view
platform.audit.export
platform.security.manage
platform.incidents.view
platform.incidents.manage
platform.feature_flags.view
platform.feature_flags.manage
platform.reports.view
platform.reports.export
```

Permission rules:

- frontend permissions are never trusted
- backend must enforce every permission
- high-risk permissions should require elevated role
- permission changes must be audited

## 12. Support Ticket Operations

Super Admin and support staff can manage organization support requests.

Required features:

- create support ticket
- view support ticket
- assign support ticket
- update support ticket status
- add internal note
- add public response placeholder
- link ticket to organization
- request support access from ticket
- escalate ticket
- close ticket

Ticket statuses:

```text
open
in_progress
waiting_for_organization
escalated
resolved
closed
```

## 13. Audit Logs

Audit logs are required for platform safety.

Required features:

- view platform audit logs
- filter by actor
- filter by organization
- filter by action
- filter by module
- filter by date range
- view before/after metadata where safe
- export audit logs with permission
- audit support access actions
- audit provider configuration changes
- audit subscription changes
- audit organization suspension/reactivation

Audit examples:

```text
audit.platform.organization.suspended
audit.platform.organization.reactivated
audit.platform.subscription.changed
audit.platform.module.enabled_for_organization
audit.platform.communication_credits.added
audit.platform.payment_provider.updated
audit.platform.support_access.approved
audit.platform.support_access.revoked
audit.platform.role.assigned
audit.platform.report.exported
```

## 14. Security And Compliance Controls

Required features:

- view failed logins
- view suspicious activity
- force password reset
- revoke user sessions
- disable compromised platform account
- review platform admin role changes
- review support access history
- review sensitive exports
- manage MFA policy placeholder
- view device/session history placeholder

Rules:

- Super Admin roles are sensitive
- dangerous actions require confirmation
- sensitive exports must be audited
- support access must be visible and reviewable
- security events must be stored separately from normal activity where needed

## 15. System Configuration

Super Admin controls global configuration.

Required features:

- platform name
- default country
- default language
- default currency
- default trial period
- default plan
- maintenance mode
- upload limits
- storage limits
- rate limits
- notification defaults
- public registration settings
- support contact settings

Configuration rules:

- global settings must not override tenant-specific settings unless intended
- config changes must be audited
- dangerous config changes must be restricted to high-level roles

## 16. Feature Flags And Rollout Control

Feature flags allow safe release management.

Required features:

- create feature flag
- enable feature globally
- disable feature globally
- enable feature for selected organizations
- enable feature by plan
- enable feature by country
- enable beta feature
- rollback feature
- view flag history

Example:

```text
Enable Pledges for five pilot churches first.
After validation, enable it for Professional plan churches.
```

## 17. Platform Monitoring

Super Admin must see platform health.

Required monitoring areas:

- API health
- database health
- Redis health
- worker health
- queue health
- payment provider health
- SMS provider health
- Email provider health
- WhatsApp provider health
- failed jobs
- slow requests
- error rate
- backup status

Health statuses:

```text
healthy
degraded
down
unknown
```

## 18. Incident Management

Super Admin manages platform incidents and affected services.

Required features:

- create incident
- update incident
- assign incident owner
- set affected service
- set affected countries
- set affected organizations placeholder
- post internal incident update
- notify affected organizations placeholder
- resolve incident
- create post-incident review placeholder

Incident statuses:

```text
investigating
identified
monitoring
resolved
```

Example incident:

```text
Payment callbacks delayed for Ghana.
Affected service: Religious Giving Payments.
Affected provider: Hubtel.
Status: Investigating.
```

## 19. Backup And Recovery Visibility

MVP may use manual recovery, but Super Admin must still see backup readiness.

Required features:

- view latest backup time
- view backup status
- view failed backups
- view backup location metadata placeholder
- view restore readiness notes
- trigger manual backup placeholder
- record restore test placeholder

Rules:

- backup failures must be visible
- restore actions must be audited
- production recovery actions must require elevated permission

## 20. Content And Public Page Governance

Super Admin may govern platform-level public content and abusive organization pages.

Required features:

- manage platform announcement banners
- manage platform terms and privacy links
- disable abusive public organization page
- review reported public page placeholder
- manage global CMS content placeholder

Rule:

```text
Organization-owned content belongs to the organization.
Super Admin governance is for platform safety, abuse prevention, and legal/compliance operations.
```

## 21. Reports And Analytics

Super Admin needs platform-level business and operational reports.

Required reports:

- organization growth report
- active organization report
- subscription revenue report
- trial conversion report
- churn risk placeholder
- payment volume report
- communication credit sales report
- SMS usage report
- Email usage report
- WhatsApp usage report
- module usage report
- country usage report
- support ticket report
- incident report
- provider failure report
- audit export report

Report rules:

- operational reports should not expose private member-level data unless explicitly required and authorized
- exports must be audited
- sensitive reports require elevated permission

## 22. Tenant Safety Controls

Super Admin must help protect tenant isolation.

Required features:

- view tenant ownership metadata
- detect orphan records placeholder
- detect cross-tenant risk placeholder
- review tenant data export requests placeholder
- block unsafe support access
- view tenant isolation check results placeholder

Critical rule:

```text
One organization must never see another organization's data.
```

## 23. Developer API And Integration Management Later

Not required for MVP, but the control plane should allow later extension.

Future features:

- view API clients
- create organization API client
- revoke API client
- view webhook endpoints
- view webhook failures
- view API usage
- apply API rate limits

## 24. Partner And Module Marketplace Management Later

Not required for MVP.

Future features:

- approve implementation partners
- approve third-party modules
- review module submissions
- manage partner referrals
- track partner performance
- partner commission placeholder

## Data Entities

Suggested platform entities:

```text
PlatformStaffUser
PlatformRole
PlatformPermission
OrganizationTenant
OrganizationSubscription
PlatformPlan
PlatformModule
OrganizationModuleEntitlement
CommunicationPackage
OrganizationCommunicationWalletSummary
PaymentProviderConfiguration
CommunicationProviderConfiguration
SupportTicket
SupportAccessRequest
PlatformAuditLog
PlatformFeatureFlag
PlatformIncident
PlatformHealthCheck
PlatformConfiguration
PlatformReportExport
```

## API Direction

Example routes:

```text
GET    /api/v1/platform/dashboard

GET    /api/v1/platform/organizations
POST   /api/v1/platform/organizations
GET    /api/v1/platform/organizations/{organizationId}
PATCH  /api/v1/platform/organizations/{organizationId}
POST   /api/v1/platform/organizations/{organizationId}/suspend
POST   /api/v1/platform/organizations/{organizationId}/reactivate

GET    /api/v1/platform/plans
POST   /api/v1/platform/plans
PATCH  /api/v1/platform/plans/{planId}

GET    /api/v1/platform/modules
POST   /api/v1/platform/modules
PATCH  /api/v1/platform/modules/{moduleId}
POST   /api/v1/platform/organizations/{organizationId}/modules/{moduleId}/enable
POST   /api/v1/platform/organizations/{organizationId}/modules/{moduleId}/disable

GET    /api/v1/platform/communication/packages
POST   /api/v1/platform/communication/packages
POST   /api/v1/platform/organizations/{organizationId}/communication/top-up

GET    /api/v1/platform/providers/payments
POST   /api/v1/platform/providers/payments
PATCH  /api/v1/platform/providers/payments/{providerId}

GET    /api/v1/platform/providers/communication
POST   /api/v1/platform/providers/communication
PATCH  /api/v1/platform/providers/communication/{providerId}

GET    /api/v1/platform/support/tickets
POST   /api/v1/platform/support/tickets
PATCH  /api/v1/platform/support/tickets/{ticketId}
POST   /api/v1/platform/support/access-requests
POST   /api/v1/platform/support/access-requests/{requestId}/approve
POST   /api/v1/platform/support/access-requests/{requestId}/revoke

GET    /api/v1/platform/audit-logs
GET    /api/v1/platform/security/events
GET    /api/v1/platform/health
GET    /api/v1/platform/incidents
POST   /api/v1/platform/incidents
PATCH  /api/v1/platform/incidents/{incidentId}

GET    /api/v1/platform/feature-flags
POST   /api/v1/platform/feature-flags
PATCH  /api/v1/platform/feature-flags/{flagId}

GET    /api/v1/platform/reports/organizations
GET    /api/v1/platform/reports/subscriptions
GET    /api/v1/platform/reports/communication-usage
GET    /api/v1/platform/reports/payment-volume
```

## Events

Platform events:

```text
platform.organization.created
platform.organization.approved
platform.organization.suspended
platform.organization.reactivated
platform.subscription.assigned
platform.subscription.updated
platform.module.enabled
platform.module.disabled
platform.communication_package.created
platform.communication_wallet.topped_up
platform.payment_provider.updated
platform.communication_provider.updated
platform.support_ticket.created
platform.support_access.requested
platform.support_access.approved
platform.support_access.revoked
platform.feature_flag.enabled
platform.feature_flag.disabled
platform.incident.created
platform.incident.resolved
platform.report.exported
```

## Audit Events

Audit events:

```text
audit.platform.organization.created
audit.platform.organization.updated
audit.platform.organization.suspended
audit.platform.organization.reactivated
audit.platform.subscription.changed
audit.platform.module.enabled_for_organization
audit.platform.module.disabled_for_organization
audit.platform.communication_package.changed
audit.platform.communication_wallet.adjusted
audit.platform.payment_provider.changed
audit.platform.communication_provider.changed
audit.platform.support_access.requested
audit.platform.support_access.approved
audit.platform.support_access.revoked
audit.platform.staff_role.assigned
audit.platform.staff_role.revoked
audit.platform.feature_flag.changed
audit.platform.incident.changed
audit.platform.report.exported
```

## MVP Scope

Build first:

```text
1. Platform dashboard
2. Organization management
3. Plan and subscription management
4. Module enable/disable
5. Communication package management
6. Payment transaction overview
7. SMS/Email/WhatsApp usage overview
8. Platform staff/admin management
9. Support ticket basics
10. Support access request/approval
11. Audit logs
12. System health dashboard
13. Feature flags
```

Delay until later:

```text
Advanced partner marketplace
Third-party module app store
Advanced AI analytics
Complex commission settlement
Full tax automation
Advanced data warehouse
Automatic legal compliance engine
Full self-service developer portal
```

## Final Rule

The Super Admin must be powerful enough to operate Hulnex safely, but not so unrestricted that it violates tenant privacy.

```text
Powerful platform control.
Strict tenant protection.
Full auditability.
```
