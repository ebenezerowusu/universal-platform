# Sprint 33 Reporting Analytics Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 33.

Sprint 33 prepares the Reporting and Analytics foundation after search, notifications, documents, commerce, HR, calendar, finance, religious, and operations modules have introduced many reportable records.

## Prompt

```text
You are implementing Sprint 33 for Universal Platform.

Your task is to implement the Reporting and Analytics foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/04-capability-engines/reporting-engine.md
3. docs/04-capability-engines/permission-engine.md
4. docs/04-capability-engines/audit-engine.md
5. docs/04-capability-engines/import-export-engine.md
6. docs/14-roadmap/sprint-33-reporting-analytics-roadmap.md
7. docs/13-module-sdk/reporting-analytics-readiness.md
8. docs/12-api/reporting-analytics-api-contract.md
9. docs/11-ui-ux/reporting-analytics-ui-contract.md
10. docs/05-infrastructure/reporting-data-quality-operations-guide.md
11. docs/16-quality/sprint-33-reporting-analytics-review.md

Implement only:
- reporting feature registration foundation
- report definition metadata foundation
- dashboard card/widget metadata foundation
- metric definition foundation
- report filter foundation
- report result snapshot placeholder where practical
- export request placeholder where practical
- scheduled report placeholder where practical
- data-quality warning foundation
- permission and feature checks
- audit records for sensitive reports/exports where required
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- full BI/OLAP warehouse
- advanced charting engine
- external analytics provider integration
- AI-generated insights
- unrestricted cross-module reporting
- sensitive report exports without permissions
- provider-specific reporting calls inside domain modules
- unrelated Commerce/POS expansion

Recommended branch:
feature/reporting-analytics-foundation

Recommended PR title:
feat: add reporting analytics foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Reporting/Permission/Export boundaries followed
- data-quality rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused reporting foundation that defines dashboards, metrics, reports, filters, snapshots, export placeholders, and data-quality warnings while preserving permissions.

## Final Rule

Reporting must be organization-scoped, permission-aware, data-quality aware, and reusable across domains. Domain modules should publish reportable metrics instead of building isolated reporting stacks.
