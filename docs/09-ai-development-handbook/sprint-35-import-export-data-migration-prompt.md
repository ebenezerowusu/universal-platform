# Sprint 35 Import Export Data Migration Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 35.

Sprint 35 prepares the Import, Export, and Data Migration foundation after reporting, documents, search, workflow, HR, commerce, religious, and finance modules have introduced bulk data movement needs.

## Prompt

```text
You are implementing Sprint 35 for Universal Platform.

Your task is to implement the Import, Export, and Data Migration foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/04-capability-engines/import-export-engine.md
3. docs/04-capability-engines/storage-engine.md
4. docs/04-capability-engines/permission-engine.md
5. docs/04-capability-engines/audit-engine.md
6. docs/14-roadmap/sprint-35-import-export-data-migration-roadmap.md
7. docs/13-module-sdk/import-export-data-migration-readiness.md
8. docs/12-api/import-export-data-migration-api-contract.md
9. docs/11-ui-ux/import-export-data-migration-ui-contract.md
10. docs/05-infrastructure/import-export-data-quality-operations-guide.md
11. docs/16-quality/sprint-35-import-export-data-migration-review.md

Implement only:
- import/export feature registration foundation
- import template metadata foundation
- export request foundation
- import job foundation
- validation result foundation
- field mapping placeholder where practical
- dry-run/preview placeholder where practical
- import execution placeholder where practical
- rollback note placeholder where practical
- import/export file reference through Document/Storage boundaries
- permission and feature checks
- audit records for import/export actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- unrestricted direct database imports
- automatic destructive migrations
- full ETL platform
- external data warehouse integration
- AI data cleaning
- cross-organization imports
- provider-specific file access inside domain modules
- unrelated Commerce/POS expansion

Recommended branch:
feature/import-export-data-migration-foundation

Recommended PR title:
feat: add import export data migration foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Import/Export/Storage boundaries followed
- validation and audit rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused import/export foundation for templates, jobs, validation, previews, export requests, file references, and audit-safe data movement.

## Final Rule

Imports and exports must be organization-scoped, permission-aware, auditable, validation-first, and routed through Document/Storage boundaries. Domain modules should define supported templates instead of importing arbitrary data directly.
