# Search Discovery UI Contract

## Purpose

This document defines the MVP UI contract for Search and Discovery foundation.

The screens should help organization users search approved records across modules while respecting permissions, visibility, and safe result fields.

## Navigation Direction

Search should be available when:

- user is authenticated
- organization is selected
- search feature is available
- user has required permission
- feature availability allows search

## Global Search Entry

Should support:

- search input
- type filter
- module filter
- date filter placeholder
- status filter placeholder
- submit/search action

## Search Results

Should show:

- result title
- safe snippet
- entity type
- module label
- status where useful
- route/action hint where allowed

## Result Filters

Should support filtering by:

- entity type
- module/domain
- status
- date range placeholder
- tag placeholder where available

## No Results State

Should show a helpful no-results message.

Do not expose whether restricted records exist.

## Access Denied State

If a result becomes inaccessible after indexing, hide it or show a generic unavailable state.

Do not show sensitive metadata from inaccessible records.

## Indexing Warning Placeholder

Admin users may see warnings such as:

- index stale
- reindex needed
- source module unavailable
- unsupported entity type

## Recent Search Placeholder

Show recent searches only when policy allows storing recent searches.

Do not store sensitive query text by default.

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- searching progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 31:

- AI semantic search UI
- vector search UI
- public cross-organization search
- advanced ranking controls
- unrestricted sensitive-record search

## Final Rule

Search UI should help users find what they are allowed to access, without revealing restricted records through labels, snippets, counts, or error states.
