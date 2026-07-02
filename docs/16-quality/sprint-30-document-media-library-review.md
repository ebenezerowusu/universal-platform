# Sprint 30 Document Media Library Review

## Purpose

This document defines review checks for Sprint 30.

Sprint 30 implements Document and Media Library foundation.

## Review 1: Scope

Sprint 30 may include:

- document/media feature registration foundation
- library/folder foundation where practical
- document metadata foundation
- storage reference foundation
- domain attachment/link foundation
- access level and visibility foundation
- file type/category foundation
- upload intent placeholder where practical
- download/view permission checks
- audit records for important document actions
- API and UI foundations

Sprint 30 should not include:

- live virus scanning engine
- full OCR or document AI
- public CDN optimization
- advanced video streaming
- e-signature workflows
- unlimited public file sharing
- provider-specific storage calls inside domain modules
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Document Media Library owns metadata and attachments
- Storage Engine owns provider-specific storage behavior
- Permission Engine owns access decisions
- Audit Engine records important actions
- domain modules do not store provider-specific paths directly

## Review 3: Organization Boundaries

Confirm:

- document records are organization-owned
- folders are organization-owned
- domain attachments cannot cross organization boundaries
- access checks use organization membership and permission rules

## Review 4: Access Rules

Confirm:

- private documents require explicit permission
- restricted documents are hidden unless allowed
- public placeholder behavior is not unrestricted
- archived or deleted placeholder documents are not shown as active

## Review 5: Storage Rules

Confirm:

- storage references are provider-neutral where possible
- clients do not receive provider credentials
- upload intent goes through platform API
- missing storage reference is handled safely

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/document-media-library-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/document-media-library-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- folder create/update
- document metadata create/update
- storage reference attachment
- domain attachment creation
- cross-organization attachment rejection
- access denied behavior
- archived document behavior
- upload intent placeholder
- permission denied behavior

## Final Rule

Sprint 30 is complete when organizations can manage document/media metadata, link records to domain workflows, and enforce access through Storage, Permission, and Audit Engine boundaries.
