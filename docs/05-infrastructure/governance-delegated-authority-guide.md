# Governance Delegated Authority Guide

## Purpose

This document defines operating guidance for governance policies, authority mandates, delegated approvers, approval thresholds, domain action mappings, temporary delegations, conflict warnings, decision logs, and governance audit history.

The goal is to define decision rights without bypassing permissions, workflow rules, organization ownership, or audit history.

## Principle

Permission is not the same as authority.

Permission controls access to platform capabilities.

Governance authority controls whether a permitted actor is the approved decision-maker for a sensitive business action.

## Data Sources

Governance authority operations may use:

- governance policy metadata
- authority mandate metadata
- delegated approver assignment placeholder
- approval threshold rule placeholder
- domain action authority mapping placeholder
- temporary delegation placeholder
- authority conflict warning placeholder
- authority decision log placeholder
- audit records

## Policy Direction

Governance policies should capture:

```text
policy_key
organization_id
domain_module_owner
policy_type
status
effective_date_placeholder
```

## Authority Mandate Direction

Authority mandates should capture:

```text
mandate_key
actor_or_role_reference
authority_scope
domain_action_type
status
start_end_placeholder
```

## Delegation Direction

Delegation placeholders may support:

- direct delegation
- role-based delegation
- branch/network-scoped delegation
- temporary delegation
- fallback approver placeholder

## Threshold Direction

Threshold placeholders may define authority by:

- amount
- branch or hierarchy level
- module/domain
- risk level
- privacy sensitivity
- support escalation severity

## Domain Action Mapping Direction

Domain action mappings may include:

- finance approval
- HR approval
- data export approval
- branch relationship approval
- provider connection approval
- workflow decision
- support escalation

## Temporary Delegation Direction

Temporary delegations should include:

- delegate
- delegator
- start/end placeholder
- reason safe summary
- status

Expired delegations should not remain active.

## Conflict Warning Direction

Warn or flag when:

- actor approves own request
- delegation is expired
- mandate conflicts with another mandate
- fallback approver is missing
- cross-organization authority is not allowed
- threshold rule is ambiguous

## Workflow Direction

Governance authority should be evaluated during workflow decision steps where practical.

Workflow remains responsible for process state transitions.

Governance authority only answers whether the actor is an approved decision-maker.

## Organization Network Direction

Cross-organization authority should require:

- active organization relationship
- explicit data sharing or governance policy
- permission check
- audit record

## Audit Direction

Audit should record:

- governance policy created or updated
- authority mandate created or updated
- delegation created or revoked placeholder
- threshold rule changed
- domain action mapping changed
- temporary delegation activated/expired placeholder
- authority conflict reviewed placeholder
- authority decision evaluated placeholder

## Final Rule

Governance authority operations must keep decision rights explicit, time-bound where needed, conflict-aware, and auditable.
