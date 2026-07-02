# Product Analytics Usage Insights Adoption Telemetry UI Contract

## Purpose

This document defines the MVP UI contract for Product Analytics, Usage Insights, and Adoption Telemetry foundation.

The screens should help permitted users review privacy-safe adoption, engagement, onboarding, activation, retention, rollout, feedback, and support correlation summaries.

## Navigation Direction

Product analytics screens should appear when:

- user is authenticated
- organization is selected where required
- product analytics feature is available
- user has required permission
- privacy and aggregation rules allow visibility

## Product Analytics Dashboard

Should show:

- tracked feature count
- active module count
- onboarding completion rate placeholder
- feature adoption warning count
- support correlation warning count
- recent analytics audit events

## Feature Adoption Summary

Should show:

- feature safe label
- rollout status placeholder
- enabled organization/user count placeholder
- viewed count placeholder
- used count placeholder
- completed count placeholder
- abandoned count placeholder
- privacy restriction indicator where applicable

## Module Engagement Summary

Should show:

- module safe label
- active users count placeholder
- active organizations count placeholder
- key workflow completion count placeholder
- trend placeholder
- engagement warning indicator

## Onboarding Funnel View

Should show funnel stages for:

- account created
- organization configured
- first user invited
- first module enabled
- first successful workflow
- onboarding completed placeholder

## Activation and Retention Summary

Should show:

- activated organization count placeholder
- returning user count placeholder
- dormant organization count placeholder
- feature retention placeholder
- insufficient sample indicator where applicable

## Feature Flag Adoption Linkage

Should show:

- feature flag safe label
- rollout segment summary
- adoption summary
- configuration link placeholder
- release/announcement link placeholder

## Feedback and Support Correlation Placeholder

Should show:

- feature/module safe label
- related feedback count placeholder
- related support case count placeholder
- knowledge base gap indicator
- product communication link where available
- reviewed status placeholder

## Aggregation Rules Screen

Should show:

- aggregation rule safe label
- minimum sample threshold placeholder
- masking status
- status
- last updated safe label

## Analytics Audit History

Should show:

- event type
- actor safe label
- target safe label
- timestamp
- safe metadata summary

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied
- restricted by privacy
- insufficient sample

## MVP Exclusions

Do not implement in Sprint 61:

- raw user activity replay UI
- cross-tenant user-level analytics UI
- surveillance-style session viewer UI
- AI-only product decision UI
- replacement reporting dashboard UI
- individual behavior export UI

## Final Rule

Product analytics UI should show aggregated adoption learning and hide anything that would expose raw user behavior or violate tenant privacy.
