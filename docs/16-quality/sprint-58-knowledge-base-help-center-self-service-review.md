# Sprint 58 Knowledge Base Help Center Self Service Review

## Purpose

This document defines review checks for Sprint 58.

Sprint 58 implements Knowledge Base, Help Center, and Self-Service Support foundation.

## Review 1: Scope

Sprint 58 may include:

- knowledge base feature registration foundation
- help article metadata foundation
- article version placeholder foundation
- help category/collection foundation
- visibility/audience rule placeholder where practical
- search indexing linkage placeholder where practical
- support escalation linkage placeholder where practical
- article feedback/usefulness placeholder foundation
- localization placeholder where practical
- related policy/training link placeholder where practical
- permission and feature checks
- audit records for help content and self-service support actions
- API and UI foundations

Sprint 58 should not include:

- full CMS replacement
- AI support bot as source of truth
- unrestricted public help publishing
- bypass of document or policy permissions
- cross-organization private article visibility
- arbitrary script widgets in help articles
- replacement of Support Operations
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Knowledge Base Foundation owns help articles, versions, categories, collections, visibility rules, feedback, related content links, and help audit history
- Document/Media Foundation owns document/file references used by help content
- Search/Discovery Foundation owns indexing and search result delivery
- Support Operations owns support cases, escalations, and agent workflows
- Policy Library owns policy and training references
- Permission Engine controls help content visibility and management rights
- Audit Engine records help content changes and support escalation links

## Review 3: Visibility and Permission Rules

Confirm:

- private organization articles do not cross organizations
- visibility rules are applied before returning articles
- linked documents respect document/media permissions
- related policy/training links respect policy library permissions
- search results respect article visibility

## Review 4: Self-Service and Escalation Rules

Confirm:

- article feedback can identify unresolved issues
- support escalation preserves article and organization context
- self-service does not replace support operations
- no-result search can direct users to support escalation where practical
- article feedback does not expose sensitive user information unnecessarily

## Review 5: Content Quality Rules

Confirm warnings exist where practical for:

- published article has no active version
- article has no category
- article has no visibility rule
- article feedback says content is outdated
- search index link is missing
- related document is no longer accessible
- support escalations from article are high

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/knowledge-base-help-center-self-service-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/knowledge-base-help-center-self-service-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- article creation/update
- article version creation/publish placeholder
- category/collection creation/update
- visibility rule enforcement
- search linkage placeholder creation
- feedback creation/review
- support escalation link creation
- related content link creation/update
- organization boundary enforcement
- permission denied behavior
- audit record creation

## Final Rule

Sprint 58 is complete when help articles, versions, categories, collections, visibility rules, search linkage, support escalation links, feedback, related content links, and help audit history are available through organization-aware, visibility-aware, search-aware, support-linked, permission-protected, and auditable foundations.
