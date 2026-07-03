# Audit Event Catalogue

## Purpose

This document consolidates audit event categories across Universal Platform foundations.

Audit events are accountability records. They are not workflow events, analytics events, or notification messages.

## Audit Naming Standard

Use this format:

```text
<audit_area>.<resource>.<action>
```

Examples:

```text
audit.permission.role_updated
audit.payment.reconciliation_reviewed
```

## Audit Rules

Audit records should include:

```text
id
organization_id where applicable
actor_user_id
actor_type
action
target_type
target_id
safe_summary
metadata
ip_address placeholder
device/session reference placeholder
created_at
```

Never store raw secrets, full payment credentials, or sensitive private payloads inside audit metadata.

## Core Audit Events

```text
audit.identity.user_created
audit.identity.user_updated
audit.identity.user_deactivated
audit.organization.created
audit.organization.updated
audit.organization_suspended
audit.membership.created
audit.membership.updated
audit.membership_revoked
audit.invitation.sent
audit.invitation.accepted
audit.session.revoked
audit.device.deactivated
```

## Permission And Security Audit Events

```text
audit.permission.role_created
audit.permission.role_updated
audit.permission.role_deleted_placeholder
audit.permission.assignment_created
audit.permission.assignment_revoked
audit.security.session_reviewed
audit.security.device_reviewed
audit.security.risk_warning_reviewed
audit.security.access_revoked
```

## Module, Subscription, And Configuration Audit Events

```text
audit.module.installed
audit.module.enabled
audit.module.disabled
audit.module.entitlement_changed
audit.subscription.plan_changed
audit.subscription.status_changed
audit.billing.record_reviewed
audit.configuration.value_changed
audit.feature_flag.created
audit.feature_flag.updated
audit.runtime_rule.changed
```

## Provider, Payment, SMS, And Communication Audit Events

```text
audit.provider_connection.created
audit.provider_connection.updated
audit.provider_connection.health_reviewed
audit.provider_credential.reference_changed
audit.payment.transaction_reviewed
audit.payment.reconciliation_reviewed
audit.payment.provider_operation_reviewed
audit.sms.wallet_adjusted
audit.sms.package_created
audit.sms.message_reviewed
audit.communication.template_changed
audit.notification.preference_changed
```

## Religious And Finance Audit Events

```text
audit.religious.member_created
audit.religious.member_updated
audit.religious.member_status_changed
audit.religious.household_link_reviewed
audit.religious.attendance_marked
audit.religious.attendance_edited
audit.religious.giving_recorded
audit.finance.income_recorded
audit.finance.expense_created
audit.finance.expense_approved
audit.finance.expense_rejected
audit.finance.budget_changed
audit.finance.report_exported
```

## Commerce, HR, Scheduling, Documents, And Reporting Audit Events

```text
audit.commerce.product_changed
audit.commerce.inventory_adjusted
audit.commerce.sale_reviewed
audit.marketplace.order_status_changed
audit.fulfillment.task_changed
audit.logistics.assignment_changed
audit.hr.staff_changed
audit.hr.leave_reviewed
audit.calendar.event_changed
audit.document.uploaded
audit.document.permission_changed
audit.document.archived
audit.report.generated
audit.report.exported
```

## Workflow, Jobs, Import, Export, And Migration Audit Events

```text
audit.workflow.definition_changed
audit.workflow.approval_recorded
audit.workflow.task_completed
audit.job.retry_requested
audit.schedule.changed
audit.import.job_created
audit.import.validation_reviewed
audit.export.job_created
audit.export.downloaded_placeholder
audit.migration.run_created
audit.migration.runbook_reviewed
audit.migration.validation_reviewed
```

## Data Governance Audit Events

```text
audit.privacy.classification_changed
audit.privacy.consent_recorded
audit.privacy.consent_withdrawn
audit.privacy.access_purpose_logged
audit.privacy.export_reviewed
audit.data_quality.rule_changed
audit.data_quality.duplicate_reviewed
audit.data_quality.merge_reviewed
audit.data_quality.correction_recorded
audit.lineage.record_linked
audit.record_history.created
audit.portability.package_requested
audit.portability.package_generated
audit.tenant_exit.reviewed
```

## Organization Network And Governance Audit Events

```text
audit.organization_network.connection_requested
audit.organization_network.connection_reviewed
audit.organization_network.disconnect_reviewed
audit.organization_network.sharing_policy_changed
audit.governance.policy_changed
audit.governance.authority_mandate_changed
audit.governance.delegation_changed
audit.governance.decision_recorded
```

## Risk, Compliance, Policy, And Knowledge Audit Events

```text
audit.risk.item_changed
audit.control.check_reviewed
audit.evidence.item_attached
audit.remediation.action_changed
audit.policy.document_changed
audit.policy.version_published
audit.attestation.requested
audit.attestation.completed
audit.training.acknowledged
audit.knowledge.article_changed
```

## Support, Product, Analytics, And Experimentation Audit Events

```text
audit.support.ticket_created
audit.support.ticket_updated
audit.support.ticket_escalated
audit.product_announcement.published
audit.release_note.published
audit.feedback.item_reviewed
audit.feature_request.changed
audit.analytics.usage_event_definition_changed
audit.analytics.aggregation_rule_changed
audit.analytics.dashboard_accessed_placeholder
audit.experiment.created
audit.experiment.variant_changed
audit.experiment.decision_reviewed
```

## Customer Success, Implementation, And Blueprint Audit Events

```text
audit.customer_success.account_health_changed
audit.customer_success.risk_signal_reviewed
audit.customer_success.playbook_changed
audit.customer_success.intervention_changed
audit.implementation.project_changed
audit.implementation.milestone_changed
audit.implementation.readiness_reviewed
audit.solution_blueprint.changed
audit.solution_blueprint.version_published
audit.deployment_package.changed
audit.deployment_package.preflight_reviewed
audit.deployment_package.preview_generated
```

## Partner Audit Events

```text
audit.partner.organization_changed
audit.partner.implementer_changed
audit.partner.certification_reviewed
audit.partner.assignment_changed
audit.partner.delegation_requested
audit.partner.delegation_approved
audit.partner.delegation_rejected
audit.partner.delegation_revoked
audit.partner.scope_changed
audit.partner.quality_reviewed
audit.partner_enablement.program_changed
audit.partner_enablement.training_path_changed
audit.partner_enablement.certification_result_reviewed
audit.partner_referral.submitted
audit.partner_referral.opportunity_registered
audit.partner_referral.conflict_reviewed
audit.partner_referral.handoff_created
```

## Developer, Operator, And Commercial Audit Events

```text
audit.developer_api.client_changed
audit.webhook.subscription_changed
audit.webhook.delivery_reviewed
audit.module_submission.reviewed
audit.operator.action_performed
audit.operator.tenant_reviewed
audit.commercial.package_changed
audit.commercial.plan_addon_changed
audit.revenue.signal_reviewed
```

## Audit Severity Levels

```text
info
notice
warning
sensitive
critical
```

Use `sensitive` or `critical` for permission changes, financial review, delegated access, provider credential references, tenant exit, data export, go-live readiness, and operator actions.

## Final Rule

Every sensitive action must be auditable. Audit records must be safe to store and safe to review.
