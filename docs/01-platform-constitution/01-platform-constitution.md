# Platform Constitution

The Platform Constitution defines the permanent engineering laws of Universal Platform.

Every developer, AI coding agent, and future contributor must follow these principles.

## 1. The Platform Core Is Sacred

The Platform Core must remain stable, reusable, and domain-agnostic.

Adding a new domain must not require rewriting the Platform Core.

The core may provide capabilities, but it must not contain business logic for a specific industry.

The core understands concepts such as:

- User
- Organization
- Tenant
- Role
- Permission
- Module
- Event
- Notification
- Payment
- SMS
- Storage
- Workflow
- Configuration
- Audit Log

The core must not contain concepts such as:

- Church
- Pastor
- Member
- Product
- Shop
- Driver
- Student
- Patient
- Employee

Those concepts belong inside domains.

## 2. Everything Is Modular

Business features belong inside domains and modules.

Examples:

- Religious Domain
- Commerce Domain
- POS Module
- Finance Module
- HR Module
- Logistics Domain
- Future Domains

A module must be installable, configurable, and removable where practical.

Modules should declare their permissions, navigation, settings, events, jobs, and migrations through a manifest.

## 3. Build Engines, Not Isolated Features

When a capability can be reused across domains, build it as an engine.

Examples:

- Attendance Engine
- Payment Engine
- SMS Engine
- Notification Engine
- Workflow Engine
- Storage Engine
- Audit Engine
- Reporting Engine
- Localization Engine
- Timeline Engine
- Import/Export Engine

A church service, Bible study session, staff clock-in, and event check-in should reuse an Attendance Engine instead of creating separate attendance logic for each case.

## 4. Everything External Must Be Replaceable

External services must be accessed through interfaces and adapters.

Domains must never call providers directly.

Examples:

- Payment Engine -> Hubtel, Paystack, Flutterwave, M-Pesa adapters
- SMS Engine -> Arkessel, Hubtel, Africa's Talking, Twilio adapters
- Storage Engine -> Local Storage, S3, Cloudflare R2, Azure Blob adapters
- Email Engine -> provider adapters

Changing providers should require configuration and adapter changes, not domain logic changes.

## 5. Everything Must Be Configurable Where Possible

Avoid hardcoding business rules that may differ by organization, country, domain, or plan.

Configuration should control:

- Country
- Currency
- Timezone
- Language
- Payment providers
- SMS providers
- Terminology
- Branding
- Navigation
- Workflows
- Approval rules
- Attendance rules
- Feature flags
- Subscription limits

## 6. API First

Every capability must be exposed through APIs.

Flutter mobile, Flutter desktop, future web clients, public integrations, and third-party modules must consume the same backend APIs.

No client should receive special hidden business logic.

## 7. Domain Isolation

Domains must not directly access another domain's database tables or internal services.

Domains communicate through:

- Public application services
- Events
- Stable contracts
- Capability engines

Direct domain-to-domain coupling is not allowed unless explicitly approved in an Architecture Decision Record.

## 8. Event-Driven First

Modules should publish domain events when important business actions happen.

Examples:

- MemberCreated
- AttendanceMarked
- PaymentCompleted
- SmsDelivered
- VisitorConverted
- OrderPaid
- StaffLeaveApproved

Other modules may consume these events without the publishing module knowing who is listening.

## 9. Country Independence

The platform must not be hardcoded for Ghana or any single country.

Country-specific behavior must be configuration-driven.

Country configuration may include:

- Currency
- Timezone
- Language
- Phone format
- Payment providers
- SMS providers
- Tax rules in the future
- Address format

Ghana may use Arkessel for SMS and Hubtel/Paystack for payments, but those choices must live in provider configuration, not in domain logic.

## 10. Infrastructure Independence

Application logic must not assume where infrastructure services run.

Today PostgreSQL may run on the same VPS.

Tomorrow PostgreSQL may move to a dedicated server or managed database service.

The application must depend on configuration such as:

```env
DATABASE_URL=
REDIS_URL=
STORAGE_PROVIDER=
SMS_PROVIDER=
PAYMENT_PROVIDER=
QUEUE_PROVIDER=
```

The business logic must not change when infrastructure moves.

## 11. Technology Independence

The business architecture must outlive the technology stack.

Rust, Flutter, PostgreSQL, Redis, Docker, and Nginx are implementation choices. The platform's long-term value is in its contracts, domains, engines, and architecture.

Technology may evolve, but the business model and architecture should remain understandable and portable.

## 12. Multi-Tenant by Default

Every major data model must be designed for multi-tenancy.

Tenant isolation, authorization, reporting, audit logs, subscriptions, and module installations must all respect organization boundaries.

No feature should assume there is only one organization.

## 13. Security and Auditability Are Core Requirements

The platform must be designed with security and traceability from the beginning.

Sensitive operations must be audited.

Examples:

- Permission changes
- Payment actions
- SMS usage
- Member profile changes
- Attendance edits
- Financial records
- Provider configuration
- Module installation
- Sensitive data access

## 14. Backward Compatibility Matters

APIs must be versioned where necessary.

Database migrations must protect existing data.

Existing organizations must not break when new modules are introduced.

## 15. Never Build Only for Today

Shortcuts that damage the platform's long-term extensibility must be avoided.

The platform must be designed to grow from:

- 1 organization
- 100 organizations
- 10,000 organizations
- Millions of users

The architecture should not require a full redesign at each growth stage.

## 16. What We Will Never Do

We will never hardcode country-specific logic into the Platform Core.

We will never let domains call third-party providers directly.

We will never let one domain directly access another domain's private database tables.

We will never put business logic in the UI.

We will never bypass capability engines for convenience.

We will never expose provider credentials to organizations.

We will never build separate codebases for each customer.

We will never break architecture rules without documenting the decision in an ADR.

## Final Law

> Build the platform once. Expand it forever.
