# Governance Delegated Authority UI Contract

## Purpose

This document defines the MVP UI contract for Governance Policy, Delegated Authority, and Approval Mandates foundation.

The screens should help permitted users review governance policies, authority mandates, delegated approvers, approval thresholds, domain action mappings, temporary delegations, authority conflict warnings, decision logs, and governance audit history.

## Navigation Direction

Governance authority screens should appear when:

- user is authenticated
- organization is selected
- governance authority feature is available
- user has required permission
- feature availability allows governance authority visibility

## Governance Authority Dashboard

Should show:

- active policy count
- active mandate count
- active delegation count
- open conflict count
- recent authority decisions
- expired delegation warning count

## Policy List

Should show:

- policy key
- domain/module owner
- policy type
- status
- effective date placeholder
- action to view detail

## Authority Mandate List

Should show:

- mandate key
- actor or role safe label
- authority scope
- domain action type
- status
- start/end placeholder

## Delegated Approver List

Should show:

- delegator safe label
- delegate safe label
- delegation type
- scope summary
- status
- revoke placeholder action where permitted

## Approval Threshold Summary

Should show:

- threshold type
- domain/module
- amount/risk/branch level placeholder
- required authority safe summary
- status

## Domain Action Mapping List

Should show:

- domain action type
- required mandate summary
- workflow linkage placeholder
- risk level placeholder
- status

## Temporary Delegation List

Should show:

- delegate safe label
- reason safe summary
- start/end placeholder
- status
- activate/expire placeholder actions where permitted

## Conflict Warnings

Should show:

- warning type
- actor safe label
- affected action safe label
- severity placeholder
- status
- mark reviewed placeholder action where permitted

## Authority Decision Log

Should show:

- domain action safe label
- actor safe label
- decision result
- mandate or policy used safe label
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

## MVP Exclusions

Do not implement in Sprint 55:

- replacement workflow builder UI
- replacement permission editor UI
- automatic sensitive action approval UI
- hidden authority escalation UI
- legal governance advice UI
- arbitrary scripting UI

## Final Rule

Governance authority UI should make decision rights visible and auditable without replacing permissions or workflow approvals.
