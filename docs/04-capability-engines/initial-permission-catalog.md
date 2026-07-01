# Initial Permission Catalog

## Purpose

This document defines the first permission keys for Universal Platform.

Permission keys should be stable, predictable, and module-aware.

## Permission Format

Use this format:

```text
<domain-or-engine>.<resource>.<action>
```

Examples:

```text
organizations.users.invite
religious.members.view
payments.transactions.view
```

## Platform Permissions

```text
platform.organizations.view
platform.organizations.create
platform.organizations.suspend
platform.modules.view
platform.modules.manage
platform.subscription_plans.view
platform.subscription_plans.manage
platform.providers.view
platform.providers.manage
platform.audit_logs.view
```

## Organization Permissions

```text
organizations.profile.view
organizations.profile.update
organizations.users.view
organizations.users.invite
organizations.users.remove
organizations.roles.view
organizations.roles.manage
organizations.settings.view
organizations.settings.update
organizations.audit_logs.view
```

## Module Permissions

```text
modules.catalog.view
modules.install
modules.activate
modules.deactivate
modules.settings.update
```

## Subscription Permissions

```text
subscriptions.view
subscriptions.change_plan
subscriptions.view_usage
subscriptions.manage_billing
```

## Payment Permissions

```text
payments.transactions.view
payments.transactions.create
payments.transactions.refund
payments.reports.view
payments.providers.view
payments.providers.manage
```

## SMS Permissions

```text
sms.wallet.view
sms.wallet.credit
sms.messages.send
sms.messages.view
sms.campaigns.view
sms.campaigns.create
sms.packages.view
sms.packages.manage
sms.providers.view
sms.providers.manage
```

## Religious Member Permissions

```text
religious.members.view
religious.members.create
religious.members.update
religious.members.transfer
religious.members.import
religious.members.export
religious.members.view_sensitive
```

## Religious Visitor Permissions

```text
religious.visitors.view
religious.visitors.create
religious.visitors.update
religious.visitors.assign_follow_up
religious.visitors.convert
religious.visitors.reports.view
```

## Religious Structure Permissions

```text
religious.branches.view
religious.branches.manage
religious.congregations.view
religious.congregations.manage
religious.services.view
religious.services.manage
religious.groups.view
religious.groups.manage
religious.groups.manage_members
religious.groups.assign_leaders
```

## Religious Attendance Permissions

```text
religious.attendance.view
religious.attendance.mark
religious.attendance.edit
religious.attendance.close_session
religious.attendance.reports.view
```

## Religious Finance/Giving Permissions

```text
religious.giving.view
religious.giving.record
religious.giving.manage_categories
religious.giving.view_reports
religious.pledges.view
religious.pledges.manage
```

## Religious Communication Permissions

```text
religious.communication.view
religious.communication.send
religious.communication.schedule
religious.communication.manage_templates
religious.communication.view_reports
```

## Religious Care Permissions

```text
religious.care.view
religious.care.manage
religious.prayer.view
religious.prayer.manage
religious.counseling.view
religious.counseling.manage
religious.counseling.view_confidential
religious.welfare.view
religious.welfare.manage
religious.welfare.approve
```

## Religious Reports Permissions

```text
religious.reports.view
religious.reports.export
religious.reports.view_sensitive
```

## Scope Types

Possible permission scopes:

```text
own
own_group
branch
subtree
organization
platform
```

## Final Rule

Permissions must be checked on the backend. Frontend menu hiding is only a usability layer.
