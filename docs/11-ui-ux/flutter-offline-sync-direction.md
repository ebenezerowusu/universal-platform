# Flutter Offline Sync Direction

## Purpose

This document defines the long-term offline sync direction for Flutter clients.

Offline support is important for field work, events, attendance, visitor registration, POS later, and areas with unstable connectivity.

## Principle

Offline support should be designed carefully and introduced in phases.

Do not make every feature offline-capable in the MVP.

## MVP Direction

For early MVP, online-first behavior is acceptable.

The codebase should still avoid decisions that make offline support impossible later.

## Future Offline Candidates

Good future candidates:

- attendance marking
- visitor registration
- event check-in
- basic member lookup
- follow-up task notes
- POS sales later

Poor early candidates:

- payment confirmation
- sensitive care records
- complex financial reports
- provider configuration
- role/permission management

## Offline Record Shape

Offline queue records may need:

- local_id
- operation_type
- organization_id
- resource_type
- resource_id nullable
- payload
- status
- created_at
- synced_at nullable
- error nullable

## Sync Principles

Offline sync should:

- preserve organization context
- avoid duplicate submissions
- resolve conflicts safely
- show sync status to the user
- retry safely where practical

## Conflict Direction

Conflict rules should be defined per feature.

Examples:

- attendance duplicate should be merged or rejected safely
- visitor duplicate should be reviewed or matched
- notes should not overwrite newer server records silently

## Security Direction

Offline storage should avoid unnecessary sensitive records.

When sensitive data must be stored locally, additional protection should be considered.

## Final Rule

Offline support is a platform advantage, but it must be introduced intentionally. Start online-first, then add offline workflows feature by feature.
