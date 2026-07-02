# Sprint 31 Search Discovery Review

## Purpose

This document defines review checks for Sprint 31.

Sprint 31 implements Search and Discovery foundation.

## Review 1: Scope

Sprint 31 may include:

- search feature registration foundation
- searchable entity registry foundation
- search index metadata foundation
- indexing job placeholder where practical
- permission-aware search result filtering
- organization-scoped search API foundation
- result type/category filtering
- recent search placeholder where practical
- search audit direction where policy requires it
- API and UI foundations

Sprint 31 should not include:

- full Elasticsearch/OpenSearch deployment
- vector search or AI semantic search
- cross-tenant/global public search
- unrestricted sensitive-record search
- personal data exposure without permissions
- provider-specific search calls inside domain modules
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Search Foundation owns search metadata and indexing coordination
- domain modules own source records and safe searchable fields
- Permission Engine controls access decisions
- Platform Core does not contain domain-specific search behavior

## Review 3: Organization Boundaries

Confirm:

- search records are organization-owned
- results do not cross organization boundaries
- indexing jobs are organization-scoped
- recent search placeholders are organization/user-scoped where enabled

## Review 4: Permission Rules

Confirm:

- search verifies source module permissions before returning results
- disabled modules do not return results
- restricted records do not leak through titles, snippets, counts, or errors
- access-denied records are hidden or safely unavailable

## Review 5: Index Safety

Confirm:

- indexed title/snippet fields are safe
- stale index records are detected where practical
- archived/deleted placeholder records do not appear as active
- domain modules do not call provider-specific search APIs directly

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/search-discovery-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/search-discovery-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- global search with organization scope
- type-filtered search
- permission denied behavior
- disabled module exclusion
- restricted result hiding
- stale index behavior
- indexing job placeholder
- safe snippet behavior

## Final Rule

Sprint 31 is complete when organizations can search safe indexed metadata across approved entity types without bypassing source-module permissions.
