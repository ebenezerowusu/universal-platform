# Sprint 51 Data Quality Master Data Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 51.

Sprint 51 prepares the Data Quality, Validation, Duplicate Detection, and Master Data Governance foundation after privacy governance, import/export, reporting, user access, organization lifecycle, search, and domain modules have introduced data accuracy and deduplication needs.

## Prompt

```text
You are implementing Sprint 51 for Universal Platform.

Your task is to implement the Data Quality, Validation, Duplicate Detection, and Master Data Governance foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/07-database/07-database-strategy.md
3. docs/12-api/import-export-data-migration-api-contract.md
4. docs/12-api/reporting-analytics-api-contract.md
5. docs/12-api/data-privacy-consent-governance-api-contract.md
6. docs/14-roadmap/sprint-51-data-quality-master-data-roadmap.md
7. docs/13-module-sdk/data-quality-master-data-readiness.md
8. docs/12-api/data-quality-master-data-api-contract.md
9. docs/11-ui-ux/data-quality-master-data-ui-contract.md
10. docs/05-infrastructure/data-quality-master-data-guide.md
11. docs/16-quality/sprint-51-data-quality-master-data-review.md

Implement only:
- data quality feature registration foundation
- data quality rule metadata foundation
- validation result metadata foundation
- duplicate detection rule placeholder foundation
- duplicate candidate foundation
- safe merge review placeholder where practical
- master data entity registry placeholder where practical
- canonical record marker placeholder where practical
- data quality score placeholder where practical
- data correction request placeholder where practical
- permission and feature checks
- audit records for sensitive data quality and merge actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- automatic destructive merge without review
- automatic deletion of duplicate records
- AI-only duplicate decisions
- cross-organization duplicate visibility
- bypass of privacy masking and consent rules
- unrestricted bulk correction
- direct database editing UI
- unrelated Commerce/POS expansion

Recommended branch:
feature/data-quality-master-data-foundation

Recommended PR title:
feat: add data quality master data foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Data Quality/Privacy/Audit/Import boundaries followed
- duplicate detection and merge-review rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused data quality foundation for validation rules, validation results, duplicate detection placeholders, duplicate candidates, safe merge reviews, master data registry placeholders, canonical record markers, quality scores, correction requests, and audit-safe data quality actions.

## Final Rule

Data quality operations must be organization-scoped, privacy-aware, permission-protected, auditable, non-destructive by default, and aligned with import/export, reporting, search, and domain ownership rules.
