# Knowledge Base Help Center Self Service Guide

## Purpose

This document defines operating guidance for help articles, article versions, categories, collections, visibility rules, search linkage, support escalation links, feedback, related content links, localization placeholders, and help audit history.

The goal is to make help content discoverable, governed, and connected to support workflows.

## Principle

Self-service help should reduce support friction.

It should not replace Support Operations, Search/Discovery, Document/Media, or Policy Library.

## Data Sources

Knowledge base operations may use:

- help article metadata
- article version placeholder
- category/collection metadata
- visibility/audience rule placeholder
- search indexing link placeholder
- support escalation link placeholder
- article feedback record
- localization placeholder
- related policy/training/document link
- audit records

## Article Direction

Help article records should capture:

```text
article_key
organization_scope optional
owner_module_domain
title
category_id
status
```

## Version Direction

Article version placeholders should capture:

- version label
- content reference placeholder
- document/media reference placeholder
- published date placeholder
- replaced version placeholder
- status

## Category Direction

Categories and collections may include:

- user guides
- admin guides
- troubleshooting
- onboarding
- policy/training references
- module-specific help

## Visibility Direction

Visibility rules may include:

- platform-public placeholder
- organization-only
- role-based
- module-enabled tenants
- branch/network scoped placeholder

Private organization articles should never be visible to other organizations.

## Search Direction

Search linkage should allow articles to be discoverable through Search/Discovery.

Search results should still respect visibility and permission rules.

## Support Escalation Direction

Support escalation links should preserve:

- article reference
- user issue summary
- organization context
- support case reference placeholder
- escalation status

## Feedback Direction

Feedback records may capture:

- useful/not useful
- issue still unresolved placeholder
- suggested improvement placeholder
- content outdated placeholder
- reviewer safe label

## Related Content Direction

Related links may connect help articles to:

- policy records
- training assignments
- document/media references
- support cases
- module workflows

## Localization Direction

Localization placeholders may support translated article versions later.

Do not auto-translate official help content without review.

## Audit Direction

Audit should record:

- article created or updated
- article version created or published placeholder
- article retired placeholder
- category/collection changed
- visibility rule changed
- feedback reviewed placeholder
- support escalation link created
- related content link changed

## Data Quality Warnings

Warn or flag when:

- published article has no active version
- article has no category
- article has no visibility rule
- article feedback says content is outdated
- search index link is missing
- related document is no longer accessible
- support escalations from article are high

## Final Rule

Knowledge base operations must preserve visibility controls, search relevance, support escalation context, and audit history.
