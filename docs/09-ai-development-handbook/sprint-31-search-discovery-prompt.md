# Sprint 31 Search Discovery Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 31.

Sprint 31 prepares the Search and Discovery foundation after documents, commerce, workforce, calendar, finance, religious, and operations records have introduced many searchable entities.

## Prompt

```text
You are implementing Sprint 31 for Universal Platform.

Your task is to implement the Search and Discovery foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/04-capability-engines/permission-engine.md
3. docs/04-capability-engines/audit-engine.md
4. docs/04-capability-engines/reporting-engine.md
5. docs/14-roadmap/sprint-31-search-discovery-roadmap.md
6. docs/13-module-sdk/search-discovery-readiness.md
7. docs/12-api/search-discovery-api-contract.md
8. docs/11-ui-ux/search-discovery-ui-contract.md
9. docs/05-infrastructure/search-indexing-operations-guide.md
10. docs/16-quality/sprint-31-search-discovery-review.md

Implement only:
- search feature registration foundation
- searchable entity registry foundation
- search index metadata foundation
- indexing job placeholder where practical
- permission-aware search result filtering
- organization-scoped search API foundation
- result type/category filtering
- recent search placeholder where practical
- search audit direction where policy requires it
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- full Elasticsearch/OpenSearch deployment
- vector search or AI semantic search
- cross-tenant/global public search
- unrestricted sensitive-record search
- personal data exposure without permissions
- provider-specific search calls inside domain modules
- unrelated Commerce/POS expansion

Recommended branch:
feature/search-discovery-foundation

Recommended PR title:
feat: add search discovery foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Search/Permission boundaries followed
- organization scoping confirmed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused search foundation that can index and search approved entity metadata while enforcing permissions and organization boundaries.

## Final Rule

Search must be organization-scoped, permission-aware, provider-agnostic, and safe by default. Domain modules should publish searchable metadata instead of owning separate search engines.
