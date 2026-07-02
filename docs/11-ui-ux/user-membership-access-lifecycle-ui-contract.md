# User Membership Access Lifecycle UI Contract

## Purpose

This document defines the MVP UI contract for User Invitation, Membership, Role Assignment, and Access Lifecycle foundation.

The screens should help permitted users invite users, review memberships, assign roles, review access history, start offboarding placeholders, and review privileged access summaries.

## Navigation Direction

User access screens should appear when:

- user is authenticated
- organization is selected
- user access lifecycle feature is available
- user has required permission
- feature availability allows user access management

## User Access Dashboard

Should show:

- active member count
- pending invitation count
- privileged user count
- offboarding user count
- access review status placeholder
- recent access changes

## Invitation List

Should show:

- invited email or phone placeholder
- invited roles
- status
- invited by safe label
- expiry placeholder
- action to view detail

## Invitation Detail

Should show:

- invited contact
- invited roles
- invitation status
- expiry placeholder
- message where available
- cancel/resend placeholder actions where permitted

## Organization Member List

Should show:

- user safe label
- email or contact placeholder
- membership status
- primary role placeholder
- privileged access flag where applicable
- last activity placeholder where available

## Member Detail

Should show:

- user safe label
- membership status
- assigned roles
- permission group placeholders
- access history summary
- offboarding status placeholder

## Role Assignment Panel

Should support:

- available roles
- assigned roles
- assign role action
- remove role action
- reason field
- confirmation prompt for privileged roles

## Access History

Should show:

- event type
- actor safe label
- target user safe label
- old value placeholder
- new value placeholder
- reason where available
- timestamp

## Offboarding Placeholder

Should show:

- offboarding status
- roles to remove
- pending handover placeholder
- open task warning placeholder
- completion action where permitted

## Privileged Access Review Summary

Should show:

- privileged users
- roles/permissions summary
- last reviewed placeholder
- access review link placeholder
- warning count where available

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 44:

- hidden impersonation UI
- password reset/auth provider UI
- biometric identity verification UI
- automatic privilege escalation UI
- full HR lifecycle UI

## Final Rule

User access UI should make access changes visible, deliberate, and auditable without allowing users to bypass Permission Engine decisions.
