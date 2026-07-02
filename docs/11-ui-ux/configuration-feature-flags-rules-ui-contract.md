# Configuration Feature Flags Rules UI Contract

## Purpose

This document defines the MVP UI contract for Configuration, Feature Flags, and Runtime Rules foundation.

The screens should help permitted users review namespaces, configuration keys, defaults, tenant overrides, feature flags, rollout placeholders, runtime rule placeholders, evaluation logs, and change history.

## Navigation Direction

Configuration screens should appear when:

- user is authenticated
- organization is selected
- configuration feature is available
- user has required permission
- feature availability allows configuration visibility

## Configuration Dashboard

Should show:

- namespace count
- tenant override count
- active feature flag count
- rollout warning count
- recent configuration changes
- default/fallback warning count where available

## Namespace List

Should show:

- namespace key
- owner engine/domain
- visibility scope
- status
- action to view detail

## Configuration Key List

Should show:

- namespace
- key
- value type
- default value safe summary
- override count placeholder
- status

## Tenant Override List

Should show:

- namespace/key
- override value safe summary
- effective status
- changed by safe label
- changed date
- warning status where available

## Feature Flag List

Should show:

- flag key
- title
- owner module/domain
- default state
- rollout state
- entitlement dependency placeholder
- status

## Rollout Summary

Should show:

- rollout rule type
- target summary
- current state
- warning count
- action to view detail

## Runtime Rule Placeholder List

Should show:

- rule key
- namespace
- rule type placeholder
- status
- last evaluated placeholder
- warning count

Rules should be shown as declarative metadata, not executable scripts.

## Evaluation Logs

Should show:

- evaluation type
- namespace/key or flag
- result safe summary
- fallback used flag
- timestamp

## Change History

Should show:

- change type
- actor safe label
- target safe label
- old value safe summary
- new value safe summary
- reason where available
- timestamp

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 48:

- script editor UI
- arbitrary rule execution UI
- secrets management UI
- tax/legal rules UI
- cross-tenant configuration browser UI
- direct provider-specific configuration UI inside domains

## Final Rule

Configuration UI should make behavior changes visible, permissioned, and auditable without exposing unsafe scripting or cross-tenant controls.
