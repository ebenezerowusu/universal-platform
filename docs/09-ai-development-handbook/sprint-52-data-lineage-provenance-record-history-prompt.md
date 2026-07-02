# Sprint 52 Data Lineage Provenance Record History Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 52.

Sprint 52 prepares the Data Lineage, Provenance, and Record History foundation after data quality, import/export, audit, reporting, search, async jobs, privacy governance, and domain modules have introduced source tracking and derivation needs.

## Prompt

```text
You are implementing Sprint 52 for Universal Platform.

Your task is to implement the Data Lineage, Provenance, and Record History foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/07-database/07-database-strategy.md
3. docs/12-api/data-quality-master-data-api-contract.md
4. docs/12-api/import-export-data-migration-api-contract.md
5. docs/12-api/audit-compliance-retention-api-contract.md
6. docs/14-roadmap/sprint-52-data-lineage-provenance-record-history-roadmap.md
7. docs/13-module-sdk/data-lineage-provenance-record-history-readiness.md
8. docs/12-api/data-lineage-provenance-record-history-api-contract.md
9. docs/11-ui-ux/data-lineage-provenance-record-history-ui-contract.md
10. docs/05-infrastructure/data-lineage-provenance-record-history-guide.md
11. docs/16-quality/sprint-52-data-lineage-provenance-record-history-review.md

Implement only:
- data lineage feature registration foundation
- source system metadata foundation
- record provenance metadata foundation
- record version/history placeholder foundation
- import/job/report/export lineage linkage where practical
- transformation step metadata placeholder where practical
- derived record relationship placeholder where practical
- lineage graph summary placeholder where practical
- correction/merge lineage linkage where practical
- permission and feature checks
- audit records for sensitive lineage/history actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- full graph database engine
- automatic reconstruction of all historical states
- destructive history rewriting
- unrestricted cross-organization lineage visibility
- bypass of privacy masking rules in lineage views
- direct database log mining
- AI-generated lineage inference as source of truth
- unrelated Commerce/POS expansion

Recommended branch:
feature/data-lineage-provenance-record-history-foundation

Recommended PR title:
feat: add data lineage provenance record history foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Lineage/Audit/Privacy/Data Quality boundaries followed
- source/provenance/history rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused lineage foundation for source systems, provenance metadata, record version placeholders, import/job/report/export linkage, transformation metadata, derived record relationships, lineage graph summaries, correction/merge lineage, and audit-safe record history actions.

## Final Rule

Lineage must explain where records came from and how they changed without exposing another tenant's data, bypassing privacy rules, or pretending to be a full historical reconstruction engine.
