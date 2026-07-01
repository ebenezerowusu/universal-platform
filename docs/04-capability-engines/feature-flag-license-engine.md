# Feature Flag and License Engine Specification

## Purpose

The Feature Flag and License Engine controls feature availability, staged rollout, beta access, and commercial access checks.

It works with Module Registry and Subscription Engine but does not replace them.

## Principle

A feature is available only when all required access checks pass.

Typical checks:

```text
module installed
module active
subscription allows feature
feature flag enabled
license or plan rule allows usage
user has permission
usage is within limits
```

## Scope

The engine may support:

- feature flags
- beta access
- organization feature overrides
- staged rollout
- license checks
- plan feature checks
- usage gates
- upgrade prompts

## Feature Flag Types

Possible flag types:

- platform
- organization
- module
- user
- region
- environment

## Suggested Entities

Candidate entities:

- feature_flags
- feature_flag_rules
- organization_feature_overrides
- feature_usage_gates
- license_rules

## Relationship to Subscription Engine

Subscription Engine owns plans, billing status, and plan limits.

Feature Flag and License Engine evaluates runtime availability together with subscription and module status.

## Relationship to Module Registry

Module Registry owns module installation and activation.

Feature checks should not allow a feature if the required module is not active.

## Common Use Cases

Examples:

- enable attendance QR check-in for beta organizations
- restrict advanced reports to paid plans
- enable new Flutter screen for selected organizations
- disable a risky feature during investigation
- show upgrade prompt when a plan feature is unavailable

## API Direction

Clients may need a safe feature availability endpoint.

The response should include only safe feature visibility details, not internal rule logic.

## Required Engines

This engine should integrate with:

- Module Registry
- Subscription Engine
- Permission Engine
- Configuration Engine
- Audit Engine

## Events Published

- FeatureFlagCreated
- FeatureFlagUpdated
- OrganizationFeatureOverrideUpdated
- FeatureAccessDenied

## Testing Requirements

Tests should cover:

- feature enabled by flag
- feature disabled by flag
- module missing blocks feature
- subscription blocks feature
- permission blocks feature
- organization override behavior

## Final Rule

Feature availability must be centrally evaluated. Do not scatter feature checks across the codebase.
