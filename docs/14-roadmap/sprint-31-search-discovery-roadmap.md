# Sprint 31 Search Discovery Roadmap

## Purpose

This document defines Sprint 31 for Search and Discovery foundation.

Sprint 31 should create a reusable search layer for organization records across domains without introducing a heavy search provider or AI semantic search too early.

## Sprint Goal

Enable organizations to search approved entity metadata across documents, products, orders, workforce records, calendar items, religious records, finance records, and other domain records while enforcing permissions and organization boundaries.

## Why This Sprint

The platform now has many searchable areas:

- documents and media
- products and inventory
- marketplace orders
- workforce profiles
- calendar scheduled items
- finance records
- religious members/events/groups
- delivery and fulfillment records

Without a shared search foundation, each module will build separate search behavior.

## Work Package 1: Searchable Entity Registry

Prepare registry entries for searchable entity types:

```text
document
product
order
workforce_profile
calendar_item
finance_record
religious_member
religious_event
custom
```

## Work Package 2: Search Index Metadata

Prepare index metadata with:

- entity type
- entity id
- organization id
- title/safe label
- summary/safe snippet
- tags optional
- status
- indexed at

## Work Package 3: Indexing Job Placeholder

Prepare indexing job direction without committing to a provider.

Initial implementation may use database-backed search metadata.

## Work Package 4: Permission-Aware Search

Search must filter results using:

- organization membership
- module availability
- entity visibility
- permission rules
- domain-specific safe fields

## Work Package 5: Result Filters

Prepare filters for:

- result type
- module/domain
- status
- date range
- tag placeholder

## Work Package 6: Recent Search Placeholder

Prepare placeholder for recent search history where allowed by policy.

Do not store sensitive query history without clear rules.

## Work Package 7: Flutter Foundation

Prepare screens or placeholders for:

- global search
- result list
- result filters
- empty/no results state
- access denied result state
- indexing warning placeholder

## Out of Scope

Do not implement:

- Elasticsearch/OpenSearch deployment
- AI semantic search
- vector embeddings
- cross-tenant public search
- sensitive unrestricted search
- advanced ranking engine

## Completion Standard

Sprint 31 is complete when organizations can search safe indexed metadata across approved entity types through permission-protected APIs and UI placeholders.

## Final Rule

Search must never reveal records that the user could not access through the source module.
