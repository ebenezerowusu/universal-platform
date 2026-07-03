# Experimentation Controlled Rollouts Product Validation UI Contract

## Purpose

This document defines the MVP UI contract for Experimentation, Controlled Rollouts, and Product Validation foundation.

The screens should help permitted users review experiments, variants, rollout cohorts, success metrics, guardrails, decision reviews, feature/configuration links, analytics/feedback/support links, communication links, and experiment audit history.

## Navigation Direction

Experimentation screens should appear when:

- user is authenticated
- organization is selected where required
- experimentation feature is available
- user has required permission
- privacy and aggregation rules allow visibility

## Experimentation Dashboard

Should show:

- active experiment count
- paused experiment count
- rollout cohort count
- open decision review count
- guardrail warning count
- recent experiment audit events

## Experiment List

Should show:

- experiment key/title
- owner module/domain
- hypothesis safe summary
- target feature/configuration safe label
- status
- action to view detail

## Experiment Detail

Should show:

- hypothesis safe summary
- target feature/configuration
- rollout cohort summary
- success metrics summary
- guardrail metrics summary
- decision review status
- related announcement/release/help links where available

## Variant List

Should show:

- variant safe label
- variant type
- description safe summary
- status
- linked configuration placeholder

## Rollout Cohort Summary

Should show:

- cohort safe label
- segment type
- estimated size placeholder
- rollout percentage placeholder
- privacy restriction indicator where applicable

## Success Metric View

Should show:

- metric safe label
- linked product analytics summary
- baseline placeholder
- current value placeholder
- target placeholder
- insufficient sample indicator where applicable

## Guardrail Metric View

Should show:

- guardrail safe label
- linked monitoring/support/privacy signal
- current status
- warning threshold placeholder
- breach indicator

## Decision Review List

Should show:

- experiment safe label
- recommended action placeholder
- reviewed by safe label
- decision status
- decision notes safe summary
- review date placeholder

## Feature and Configuration Link View

Should show:

- feature flag safe label
- configuration key safe label
- rollout rule safe summary
- link status
- warning when direct mutation is not allowed

## Analytics Feedback Support Link View

Should show:

- adoption metric safe label
- related feedback count placeholder
- related support case count placeholder
- knowledge base gap placeholder
- review status

## Experiment Audit History

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

Do not implement in Sprint 62:

- automatic decision UI
- dark-pattern experiment UI
- raw activity replay UI
- cross-tenant user-level cohort UI
- feature flag mutation bypass UI
- legal/medical/financial eligibility experiment UI

## Final Rule

Experimentation UI should support safe validation review while making it clear that rollout decisions require human review and separate feature flag controls.
