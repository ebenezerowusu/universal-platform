# Subscription Engine Specification

## Purpose

The Subscription Engine manages SaaS plans, feature access, usage limits, billing state, renewals, and module availability for organizations.

It enables the platform to monetize through subscriptions, premium modules, limits, SMS packages, payment processing, and future add-ons.

## Responsibilities

The Subscription Engine is responsible for:

- Subscription plans
- Plan features
- Plan limits
- Organization subscriptions
- Trial periods
- Renewal status
- Expiration handling
- Feature access checks
- Usage limit checks
- Upgrade/downgrade support
- Subscription events
- Subscription audit support

## Non-Responsibilities

The Subscription Engine does not process payments directly.

Payment collection must use the Payment Engine.

The Subscription Engine decides whether an organization is allowed to use a feature. The Payment Engine handles financial transaction processing.

## Core Concepts

### Plan

A subscription product offered by the platform.

Examples:

- Free
- Starter
- Growth
- Premium
- Enterprise

### Feature

A capability that may be allowed or blocked by a plan.

Examples:

- religious.members
- religious.attendance.qr
- sms.enabled
- payments.enabled
- reports.advanced
- finance.enabled

### Limit

A quantity restriction.

Examples:

- max_members
- max_branches
- max_users
- max_storage_mb
- max_sms_campaign_recipients
- max_products

### Organization Subscription

The active subscription state for an organization.

## Suggested Entities

Candidate entities:

- subscription_plans
- subscription_plan_features
- subscription_plan_limits
- organization_subscriptions
- subscription_usage_counters
- subscription_invoices optional later
- subscription_events

## Plan Fields

Candidate fields:

- id
- name
- code
- description
- currency
- price
- billing_interval
- status
- trial_days
- created_at
- updated_at

## Organization Subscription Fields

Candidate fields:

- id
- organization_id
- plan_id
- status
- starts_at
- ends_at nullable
- trial_ends_at nullable
- cancelled_at nullable
- current_period_start
- current_period_end
- created_at
- updated_at

Possible statuses:

- trialing
- active
- past_due
- expired
- cancelled
- suspended

## Feature Access Flow

```text
Feature requested
  -> Resolve organization subscription
  -> Check plan status
  -> Check feature availability
  -> Check module installation if required
  -> Check usage limit if applicable
  -> Allow or deny
```

## Limit Enforcement

Limits must be checked before actions that increase usage.

Examples:

- Adding a member checks max_members.
- Creating a branch checks max_branches.
- Adding a user checks max_users.
- Sending a large SMS campaign checks allowed campaign size.

## Plan and Module Relationship

A module is usable only if:

1. The module is installed and active.
2. The subscription plan allows the module or feature.
3. The user has permission.
4. The organization is within usage limits.

## Example Plans

### Free

- Basic organization profile
- Limited members
- One branch
- Basic attendance
- No advanced reports
- No payment processing

### Premium

- More members
- Multiple branches
- SMS enabled
- Payments enabled
- Advanced reports
- Finance module access depending on package

## Upgrade Flow

```text
Organization selects new plan
  -> Payment Engine processes payment if required
  -> PaymentCompleted event received
  -> Subscription Engine updates organization subscription
  -> SubscriptionChanged event published
  -> Module/feature access updated
```

## Downgrade Flow

Downgrades must be careful when current usage exceeds the new plan's limits.

Possible policies:

- Apply downgrade at next renewal
- Block downgrade until usage is reduced
- Allow downgrade but block new usage beyond limits

The selected policy must be documented before implementation.

## Events Published

- SubscriptionCreated
- SubscriptionActivated
- SubscriptionChanged
- SubscriptionExpired
- SubscriptionCancelled
- SubscriptionRenewed
- SubscriptionLimitReached
- PlanCreated
- PlanUpdated

## Events Consumed

- PaymentCompleted
- PaymentFailed
- OrganizationCreated
- ModuleInstalled

## Security Requirements

- Only platform admins can create or change global plans.
- Organization admins may upgrade/downgrade according to platform rules.
- Subscription changes must be audited.
- Plan limits must be enforced server-side.

## Tenant Isolation

Organization subscription records must be scoped to the organization.

An organization must not see or change another organization's subscription.

## Testing Requirements

Tests must cover:

- Active subscription allows feature
- Expired subscription blocks feature
- Plan without feature blocks feature
- Limit reached blocks new action
- Upgrade unlocks feature
- Downgrade policy works
- Tenant isolation
- Permission enforcement

## Final Rule

The Subscription Engine controls commercial access. It must be checked before premium modules, restricted features, and usage-limited actions are allowed.
