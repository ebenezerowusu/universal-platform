# Event Catalogue

## Purpose

This document consolidates planned domain and platform events for Universal Platform.

Events support workflow automation, webhooks, notifications, audit correlation, analytics, and background processing.

## Event Naming Standard

Use this format:

```text
<domain>.<resource>.<past_tense_action>
```

Examples:

```text
organization.created
member.registered
payment.completed
```

## Event Rules

- Events describe facts that already happened.
- Events should include organization context where applicable.
- Events should not contain raw secrets or sensitive payloads.
- Events should reference entities by ID, not embed full records.
- Audit events and domain events are related but not the same.

## Core Events

```text
identity.user.created
identity.user.updated
identity.user.deactivated
organization.created
organization.updated
organization.suspended
organization.reactivated
membership.created
membership.updated
membership.revoked
invitation.sent
invitation.accepted
invitation.expired
session.created
session.revoked
device.registered
device.deactivated
```

## Permission, Audit, Module, And Configuration Events

```text
permission.role.created
permission.role.updated
permission.assignment.created
permission.assignment.revoked
module.installed
module.enabled
module.disabled
module.entitlement.changed
subscription.created
subscription.updated
subscription.cancelled
configuration.value.changed
feature_flag.created
feature_flag.updated
feature_flag.evaluated
runtime_rule.changed
```

## Provider, Payment, SMS, And Communication Events

```text
provider_connection.created
provider_connection.updated
provider_connection.health_checked
payment.initiated
payment.completed
payment.failed
payment.reconciled
sms.package.purchased
sms.wallet.credited
sms.message.queued
sms.message.sent
sms.message.failed
notification.created
notification.delivered
notification.read
communication.template.created
communication.campaign.placeholder_sent
```

## Religious Domain Events

```text
religious.member.registered
religious.member.updated
religious.member.status_changed
religious.visitor.recorded
religious.household.link_requested
religious.household.link_approved
religious.congregation.created
religious.group.created
religious.group.member_assigned
religious.service.created
religious.attendance_session.opened
religious.attendance_session.closed
religious.attendance.marked
religious.giving.recorded
religious.bible_study.program_created
religious.bible_study.attendance_marked
```

## Finance Events

```text
finance.income.recorded
finance.giving.recorded
finance.expense.created
finance.expense.approved
finance.expense.rejected
finance.budget.created
finance.budget.updated
finance.cash_position.generated
finance.report.generated
```

## Commerce, Marketplace, Fulfillment, And Logistics Events

```text
commerce.product.created
commerce.product.updated
commerce.inventory.adjusted
commerce.sale.completed
marketplace.order.created
marketplace.order.paid
marketplace.order.cancelled
fulfillment.task.created
fulfillment.task.completed
pickup.ready
pickup.completed
delivery.assigned
delivery.started
delivery.completed
delivery.failed
```

## HR, Scheduling, Documents, Search, And Reporting Events

```text
hr.staff.created
hr.attendance.marked
hr.leave.requested
hr.leave.approved
calendar.event.created
calendar.event.updated
booking.created
booking.cancelled
document.uploaded
document.updated
document.archived
media.uploaded
search.index.updated
report.generated
dashboard.viewed_placeholder
export.created
```

## Workflow, Jobs, Import, Export, And Migration Events

```text
workflow.created
workflow.started
workflow.step.completed
workflow.approval.requested
workflow.approval.approved
workflow.approval.rejected
job.queued
job.started
job.completed
job.failed
schedule.triggered
import.started
import.validated
import.completed
import.failed
export.started
export.completed
migration.run_created
migration.validation_completed
migration.completed_placeholder
```

## Data Governance Events

```text
privacy.consent.recorded
privacy.consent.withdrawn
privacy.access.logged
data_quality.validation.completed
data_quality.duplicate.detected
data_quality.merge_review.created
data_quality.correction.requested
lineage.record.linked
record.history.created
portability.package.requested
portability.package.generated
tenant_exit.review.created
tenant_exit.handover.completed_placeholder
```

## Organization Network And Governance Events

```text
organization_network.connection.requested
organization_network.connection.approved
organization_network.connection.rejected
organization_network.disconnect.requested
organization_network.disconnect.approved
organization_network.relationship.revoked
data_sharing.policy.changed
governance.policy.created
governance.mandate.created
governance.delegation.created
governance.decision.recorded
```

## Risk, Compliance, Policy, And Knowledge Events

```text
risk.item.created
risk.item.updated
control.check.completed
evidence.item.attached
remediation.action.created
policy.created
policy.published
attestation.requested
attestation.completed
training.acknowledged
knowledge.article.created
knowledge.article.updated
knowledge.feedback.recorded
```

## Support, Product, Analytics, And Experimentation Events

```text
support.ticket.created
support.ticket.updated
support.ticket.escalated
support.ticket.resolved
product_announcement.published
release_note.published
feedback.item.submitted
feature_request.created
roadmap_candidate.created
usage_event.recorded
analytics.summary.generated
experiment.created
experiment.started
experiment.paused
experiment.completed
experiment.decision_reviewed
```

## Customer Success, Implementation, And Blueprints Events

```text
customer_success.account_health.updated
customer_success.risk_signal.created
customer_success.playbook.started
customer_success.task.completed
customer_success.intervention.created
implementation.project.created
implementation.milestone.completed
implementation.runbook.reviewed
implementation.go_live_review.recorded
solution_blueprint.created
solution_blueprint.version_published
deployment_package.preflight_completed
deployment_package.preview_generated
deployment_package.activity_recorded
```

## Partner Events

```text
partner.organization.created
partner.implementer.created
partner.certification.reviewed
partner.assignment.created
partner.delegation.requested
partner.delegation.approved
partner.delegation.revoked
partner.quality_review.recorded
partner_enablement.program.created
partner_enablement.training_path.created
partner_enablement.certification_attempt.recorded
partner_enablement.certification_result.reviewed
partner_referral.submitted
partner_referral.opportunity_registered
partner_referral.pipeline_stage_changed
partner_referral.conflict_reviewed
partner_referral.handoff_created
```

## Developer, Operator, And Commercial Events

```text
developer_api.client.created
webhook.subscription.created
webhook.delivery.succeeded
webhook.delivery.failed
module_submission.created
module_review.completed
operator.action.performed
commercial_package.created
plan_addon.created
usage_revenue_signal.recorded
```

## Final Rule

Events are for integration and automation. Audit events are for accountability. Do not confuse the two.
