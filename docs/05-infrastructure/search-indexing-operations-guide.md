# Search Indexing Operations Guide

## Purpose

This document defines operating guidance for search metadata and indexing placeholders.

The goal is to keep search safe, organization-scoped, and provider-agnostic while allowing future movement to a dedicated search provider.

## Principle

Search should index safe metadata only.

Source domains own their records and define which fields are safe to index.

## Data Sources

Search workflows may use:

- searchable entity registry
- search index metadata
- source domain record reference
- indexing job placeholder
- recent search placeholder where allowed
- audit records

## Index Metadata Direction

Index metadata should capture:

```text
organization_id
entity_type
entity_id
title
safe_snippet
module
status
tags optional
indexed_at
```

Do not index raw sensitive content by default.

## Source Domain Direction

Each domain should publish safe searchable fields.

Examples:

- document title and category
- product name and SKU
- order number and safe status
- calendar title and date
- workforce safe display label where permitted

## Permission Direction

Before returning a result, search should verify:

- organization membership
- module availability
- entity type permission
- source record visibility
- domain-specific access rules

## Stale Index Direction

Search metadata can become stale.

Flag or reindex when:

- source record changes
- source record is archived or deleted placeholder
- access level changes
- module is disabled
- domain reference becomes invalid

## Recent Search Direction

Recent searches should be disabled by default unless policy allows it.

Do not store sensitive query text without explicit rules.

## Provider Direction

Sprint 31 should not require Elasticsearch, OpenSearch, vector DB, or AI embeddings.

Start provider-neutral so future providers can be added behind adapters.

## Audit Direction

Audit may record:

- index rebuild requested
- search metadata changed
- sensitive search where policy requires it
- admin search over restricted entity types

## Data Quality Warnings

Warn or flag when:

- indexed record source no longer exists
- indexed record points to disabled module
- indexed record is accessible in index but restricted in source
- unsafe snippet is detected by policy later

## Final Rule

Search indexing must never become a shortcut around source-module permissions.
