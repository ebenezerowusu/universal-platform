# Permission Catalogue

## Purpose

This document consolidates permission namespaces across the Universal Platform foundations.

The catalogue is a planning reference. Implementation may add exact permission rows through migrations as each module is built.

## Permission Naming Standard

Use this format:

```text
<foundation_or_module>.<resource>.<action>
```

Common actions:

```text
view
list
create
update
manage
delete
approve
reject
revoke
review
export
import
send
configure
```

Destructive actions should be rare, reviewed, and audited.

## Core Permissions

```text
identity.users.view
identity.users.manage
identity.profiles.view
identity.profiles.manage
organizations.view
organizations.manage
organizations.lifecycle.view
organizations.lifecycle.manage
memberships.view
memberships.manage
invitations.view
invitations.manage
access_lifecycle.revoke
sessions.view
sessions.manage
devices.view
devices.manage
security_controls.view
security_controls.manage
```

## Permission And Audit

```text
permissions.roles.view
permissions.roles.manage
permissions.assignments.view
permissions.assignments.manage
permissions.policies.view
permissions.policies.manage
audit.history.view
audit.history.export
audit.sensitive_actions.view
```

## Module, Subscription, And Configuration

```text
modules.registry.view
modules.registry.manage
modules.installations.view
modules.installations.manage
subscriptions.plans.view
subscriptions.plans.manage
subscriptions.billing.view
subscriptions.billing.manage
configuration.namespaces.view
configuration.namespaces.manage
configuration.keys.view
configuration.keys.manage
feature_flags.view
feature_flags.manage
runtime_rules.view
runtime_rules.manage
```

## Provider, Payment, SMS, And Communication

```text
provider_connections.view
provider_connections.manage
provider_credentials.view
provider_credentials.manage
payments.transactions.view
payments.transactions.manage
payments.reconciliation.view
payments.reconciliation.manage
sms.wallet.view
sms.wallet.manage
sms.packages.view
sms.packages.manage
sms.messages.send
sms.messages.view
communication.templates.view
communication.templates.manage
communication.governance.view
communication.governance.manage
notifications.center.view
notifications.preferences.manage
```

## Religious Domain

```text
religious.members.view
religious.members.manage
religious.visitors.view
religious.visitors.manage
religious.households.view
religious.households.manage
religious.congregations.view
religious.congregations.manage
religious.groups.view
religious.groups.manage
religious.services.view
religious.services.manage
religious.bible_study.view
religious.bible_study.manage
religious.attendance.view
religious.attendance.manage
religious.giving.view
religious.giving.manage
religious.reports.view
```

## Finance

```text
finance.income.view
finance.income.manage
finance.giving.view
finance.giving.manage
finance.expenses.view
finance.expenses.manage
finance.budgets.view
finance.budgets.manage
finance.cash_position.view
finance.reports.view
finance.reports.export
```

## Commerce, Marketplace, Fulfillment, And Logistics

```text
commerce.products.view
commerce.products.manage
commerce.inventory.view
commerce.inventory.manage
commerce.sales.view
commerce.sales.manage
commerce.pos.view
commerce.pos.manage
marketplace.catalogue.view
marketplace.orders.view
marketplace.orders.manage
fulfillment.tasks.view
fulfillment.tasks.manage
pickup.orders.view
pickup.orders.manage
logistics.deliveries.view
logistics.deliveries.manage
logistics.assignments.view
logistics.assignments.manage
```

## HR, Scheduling, Documents, Search, And Reporting

```text
hr.staff.view
hr.staff.manage
hr.attendance.view
hr.attendance.manage
hr.leave.view
hr.leave.manage
calendar.events.view
calendar.events.manage
scheduling.bookings.view
scheduling.bookings.manage
documents.library.view
documents.library.manage
media.library.view
media.library.manage
search.global.view
reports.dashboards.view
reports.dashboards.manage
reports.exports.create
```

## Data Governance

```text
data_migration.imports.view
data_migration.imports.manage
data_migration.exports.view
data_migration.exports.manage
data_quality.rules.view
data_quality.rules.manage
data_quality.duplicates.view
data_quality.duplicates.manage
data_quality.corrections.manage
privacy.classification.view
privacy.classification.manage
privacy.consent.view
privacy.consent.manage
privacy.exports.review
lineage.records.view
lineage.records.manage
portability.exports.view
portability.exports.manage
tenant_exit.reviews.view
tenant_exit.reviews.manage
```

## Organization Network And Governance

```text
organization_network.relationships.view
organization_network.relationships.manage
organization_network.sharing.view
organization_network.sharing.manage
governance.policies.view
governance.policies.manage
governance.authority.view
governance.authority.manage
governance.delegations.view
governance.delegations.manage
governance.approval_mandates.manage
```

## Risk, Compliance, Policy, And Knowledge

```text
risk_register.risks.view
risk_register.risks.manage
controls.objectives.view
controls.objectives.manage
compliance.evidence.view
compliance.evidence.manage
policy_library.policies.view
policy_library.policies.manage
policy_library.attestations.view
policy_library.attestations.manage
training.acknowledgements.view
training.acknowledgements.manage
knowledge_base.articles.view
knowledge_base.articles.manage
knowledge_base.feedback.view
knowledge_base.feedback.manage
```

## Product, Support, Analytics, And Customer Success

```text
support.tickets.view
support.tickets.manage
support.escalations.view
support.escalations.manage
product_announcements.view
product_announcements.manage
release_notes.view
release_notes.manage
feedback.items.view
feedback.items.manage
feature_requests.view
feature_requests.manage
product_analytics.dashboard.view
product_analytics.usage_events.view
product_analytics.usage_events.manage
product_analytics.adoption.view
product_analytics.engagement.view
experimentation.experiments.view
experimentation.experiments.manage
experimentation.decision_reviews.manage
customer_success.account_health.view
customer_success.account_health.manage
customer_success.playbooks.view
customer_success.playbooks.manage
customer_success.interventions.manage
```

## Implementation, Blueprints, And Partner Ecosystem

```text
implementation_delivery.projects.view
implementation_delivery.projects.manage
implementation_delivery.milestones.manage
implementation_delivery.runbooks.manage
implementation_delivery.readiness_reviews.manage
solution_blueprints.blueprints.view
solution_blueprints.blueprints.manage
solution_blueprints.templates.manage
solution_blueprints.packages.manage
solution_blueprints.preview_apply.view
partner_ecosystem.partners.view
partner_ecosystem.partners.manage
partner_ecosystem.implementers.manage
partner_ecosystem.delegations.manage
partner_ecosystem.scopes.manage
partner_enablement.programs.manage
partner_enablement.training_paths.manage
partner_enablement.certifications.manage
partner_referrals.referrals.submit
partner_referrals.referrals.manage
partner_referrals.opportunities.manage
partner_referrals.conflicts.manage
```

## Operator And Developer Ecosystem

```text
operator.dashboard.view
operator.tenants.view
operator.tenants.manage
operator.incidents.view
operator.incidents.manage
operator.billing_visibility.view
developer_api.clients.view
developer_api.clients.manage
webhooks.subscriptions.view
webhooks.subscriptions.manage
webhooks.deliveries.view
module_publishing.submissions.view
module_publishing.submissions.manage
module_publishing.reviews.manage
```

## Final Rule

Frontend permissions are only hints. Backend permission checks are the source of truth.
