# Sprint 50 Data Privacy Consent Governance Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 50.

Sprint 50 prepares the Data Privacy, Consent, and Sensitive Data Governance foundation after provider connection safety, audit/compliance, document storage, reporting/export, user access, security events, and domain modules have introduced privacy and sensitive-data handling needs.

## Prompt

```text
You are implementing Sprint 50 for Universal Platform.

Your task is to implement the Data Privacy, Consent, and Sensitive Data Governance foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/08-security/08-security-standards.md
3. docs/12-api/audit-compliance-retention-api-contract.md
4. docs/12-api/reporting-analytics-api-contract.md
5. docs/12-api/import-export-data-migration-api-contract.md
6. docs/14-roadmap/sprint-50-data-privacy-consent-governance-roadmap.md
7. docs/13-module-sdk/data-privacy-consent-governance-readiness.md
8. docs/12-api/data-privacy-consent-governance-api-contract.md
9. docs/11-ui-ux/data-privacy-consent-governance-ui-contract.md
10. docs/05-infrastructure/data-privacy-consent-governance-guide.md
11. docs/16-quality/sprint-50-data-privacy-consent-governance-review.md

Implement only:
- privacy governance feature registration foundation
- data classification metadata foundation
- sensitive field catalog placeholder where practical
- consent purpose metadata foundation
- consent record foundation
- consent withdrawal placeholder foundation
- data access purpose logging placeholder where practical
- masking/redaction policy placeholder foundation
- privacy-safe export review placeholder where practical
- subject request placeholder where practical
- permission and feature checks
- audit records for privacy and consent actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- legal advice engine
- automatic regulatory compliance decisioning
- destructive data deletion automation
- unrestricted access to sensitive records
- cross-organization privacy visibility
- automatic classification using AI
- public data broker integrations
- unrelated Commerce/POS expansion

Recommended branch:
feature/data-privacy-consent-governance-foundation

Recommended PR title:
feat: add data privacy consent governance foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Privacy/Audit/Permission/Retention boundaries followed
- sensitive data and consent rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused privacy governance foundation for data classification, sensitive field catalog placeholders, consent purposes, consent records, withdrawal placeholders, access-purpose logging, masking/redaction placeholders, privacy-safe export review, subject request placeholders, and privacy audit history.

## Final Rule

Privacy governance must be organization-scoped, permission-protected, auditable, consent-aware, export-aware, and aligned with retention rules. Domains should declare sensitive data handling through shared privacy boundaries instead of creating isolated privacy behavior.
