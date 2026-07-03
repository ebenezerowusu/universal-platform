# API Route Catalogue

## Purpose

This document consolidates API route groups across the Universal Platform foundations.

Individual API contract files remain the detailed source for request and response direction. This catalogue is for discovery and implementation planning.

## API Standard

Base pattern:

```text
/api/v1/organizations/{organizationId}/<foundation>
```

Platform-only routes may later use:

```text
/api/v1/platform/<foundation>
```

Public or unauthenticated routes must be explicitly designed and reviewed.

## System Routes

```text
GET /health
GET /ready
```

## Identity, Organization, Access, And Security

```text
/api/v1/identity/users
/api/v1/identity/profiles
/api/v1/organizations
/api/v1/organizations/{organizationId}/memberships
/api/v1/organizations/{organizationId}/invitations
/api/v1/organizations/{organizationId}/access-lifecycle
/api/v1/organizations/{organizationId}/sessions
/api/v1/organizations/{organizationId}/devices
/api/v1/organizations/{organizationId}/security-controls
```

## Permission, Audit, Module, Subscription, And Configuration

```text
/api/v1/organizations/{organizationId}/permissions/roles
/api/v1/organizations/{organizationId}/permissions/assignments
/api/v1/organizations/{organizationId}/audit-history
/api/v1/organizations/{organizationId}/modules
/api/v1/organizations/{organizationId}/module-installations
/api/v1/organizations/{organizationId}/subscriptions
/api/v1/organizations/{organizationId}/billing
/api/v1/organizations/{organizationId}/configuration
/api/v1/organizations/{organizationId}/feature-flags
/api/v1/organizations/{organizationId}/runtime-rules
```

## Providers, Payments, SMS, And Notifications

```text
/api/v1/organizations/{organizationId}/provider-connections
/api/v1/organizations/{organizationId}/payment-provider-operations
/api/v1/organizations/{organizationId}/payments
/api/v1/organizations/{organizationId}/sms-wallet
/api/v1/organizations/{organizationId}/sms-packages
/api/v1/organizations/{organizationId}/sms-messages
/api/v1/organizations/{organizationId}/communication-governance
/api/v1/organizations/{organizationId}/notifications
/api/v1/organizations/{organizationId}/notification-preferences
```

## Religious Domain

```text
/api/v1/organizations/{organizationId}/religious/members
/api/v1/organizations/{organizationId}/religious/visitors
/api/v1/organizations/{organizationId}/religious/households
/api/v1/organizations/{organizationId}/religious/congregations
/api/v1/organizations/{organizationId}/religious/groups
/api/v1/organizations/{organizationId}/religious/services
/api/v1/organizations/{organizationId}/religious/bible-study
/api/v1/organizations/{organizationId}/religious/attendance
/api/v1/organizations/{organizationId}/religious/giving
/api/v1/organizations/{organizationId}/religious/reports
```

## Finance

```text
/api/v1/organizations/{organizationId}/finance/income
/api/v1/organizations/{organizationId}/finance/giving
/api/v1/organizations/{organizationId}/finance/expenses
/api/v1/organizations/{organizationId}/finance/budgets
/api/v1/organizations/{organizationId}/finance/cash-position
/api/v1/organizations/{organizationId}/finance/reports
```

## Commerce, Marketplace, Fulfillment, And Logistics

```text
/api/v1/organizations/{organizationId}/commerce/products
/api/v1/organizations/{organizationId}/commerce/inventory
/api/v1/organizations/{organizationId}/commerce/pos
/api/v1/organizations/{organizationId}/marketplace/catalogue
/api/v1/organizations/{organizationId}/marketplace/orders
/api/v1/organizations/{organizationId}/fulfillment/tasks
/api/v1/organizations/{organizationId}/pickup/orders
/api/v1/organizations/{organizationId}/logistics/deliveries
/api/v1/organizations/{organizationId}/logistics/assignments
```

## HR, Scheduling, Documents, Search, And Reporting

```text
/api/v1/organizations/{organizationId}/hr/staff
/api/v1/organizations/{organizationId}/hr/attendance
/api/v1/organizations/{organizationId}/hr/leave
/api/v1/organizations/{organizationId}/calendar/events
/api/v1/organizations/{organizationId}/scheduling/bookings
/api/v1/organizations/{organizationId}/documents
/api/v1/organizations/{organizationId}/media
/api/v1/organizations/{organizationId}/search
/api/v1/organizations/{organizationId}/reports
/api/v1/organizations/{organizationId}/dashboards
```

## Workflow, Jobs, Import, Export, And Migration

```text
/api/v1/organizations/{organizationId}/workflows
/api/v1/organizations/{organizationId}/workflow-tasks
/api/v1/organizations/{organizationId}/jobs
/api/v1/organizations/{organizationId}/queues
/api/v1/organizations/{organizationId}/schedules
/api/v1/organizations/{organizationId}/imports
/api/v1/organizations/{organizationId}/exports
/api/v1/organizations/{organizationId}/migrations
```

## Data Governance

```text
/api/v1/organizations/{organizationId}/privacy
/api/v1/organizations/{organizationId}/consent
/api/v1/organizations/{organizationId}/data-quality
/api/v1/organizations/{organizationId}/duplicates
/api/v1/organizations/{organizationId}/lineage
/api/v1/organizations/{organizationId}/record-history
/api/v1/organizations/{organizationId}/portability
/api/v1/organizations/{organizationId}/tenant-exit
```

## Organization Network And Governance

```text
/api/v1/organizations/{organizationId}/organization-network
/api/v1/organizations/{organizationId}/organization-relationships
/api/v1/organizations/{organizationId}/data-sharing
/api/v1/organizations/{organizationId}/governance
/api/v1/organizations/{organizationId}/delegated-authority
/api/v1/organizations/{organizationId}/approval-mandates
```

## Risk, Compliance, Policy, And Knowledge

```text
/api/v1/organizations/{organizationId}/risk-register
/api/v1/organizations/{organizationId}/controls
/api/v1/organizations/{organizationId}/compliance-evidence
/api/v1/organizations/{organizationId}/policy-library
/api/v1/organizations/{organizationId}/attestations
/api/v1/organizations/{organizationId}/training-acknowledgements
/api/v1/organizations/{organizationId}/knowledge-base
/api/v1/organizations/{organizationId}/help-center
```

## Support, Product, Analytics, Experimentation, And Customer Success

```text
/api/v1/organizations/{organizationId}/support
/api/v1/organizations/{organizationId}/product-announcements
/api/v1/organizations/{organizationId}/release-notes
/api/v1/organizations/{organizationId}/feedback
/api/v1/organizations/{organizationId}/feature-requests
/api/v1/organizations/{organizationId}/product-analytics
/api/v1/organizations/{organizationId}/experimentation
/api/v1/organizations/{organizationId}/customer-success
```

## Implementation, Blueprints, Partners, And Referrals

```text
/api/v1/organizations/{organizationId}/implementation-delivery
/api/v1/organizations/{organizationId}/solution-blueprints
/api/v1/organizations/{organizationId}/partner-ecosystem
/api/v1/organizations/{organizationId}/partner-enablement
/api/v1/organizations/{organizationId}/partner-referrals
```

## Developer Ecosystem And Operator Control Plane

```text
/api/v1/organizations/{organizationId}/developer-api-clients
/api/v1/organizations/{organizationId}/webhooks
/api/v1/platform/operator
/api/v1/platform/tenants
/api/v1/platform/module-publishing
/api/v1/platform/commercial-packaging
```

## Standard Resource Actions

Most protected resources should follow this shape where practical:

```text
GET /<resource>
POST /<resource>
GET /<resource>/{id}
PATCH /<resource>/{id}
POST /<resource>/{id}/mark-reviewed-placeholder
GET /<resource>/audit-history
```

## Final Rule

Every route must enforce authentication, tenant visibility, permission checks, and audit rules where applicable.
