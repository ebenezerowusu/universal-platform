# Sprint 42 Country Localization Regional Operations Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 42.

Sprint 42 prepares the Country, Localization, and Regional Operations foundation after payment operations, subscription billing, SMS, reporting, organization onboarding, and multi-domain workflows have introduced country-specific configuration needs.

## Prompt

```text
You are implementing Sprint 42 for Universal Platform.

Your task is to implement the Country, Localization, and Regional Operations foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/00-vision/00-platform-vision.md
3. docs/01-platform-constitution/01-platform-constitution.md
4. docs/04-capability-engines/localization-engine.md
5. docs/04-capability-engines/payment-engine.md
6. docs/04-capability-engines/sms-engine.md
7. docs/14-roadmap/sprint-42-country-localization-regional-ops-roadmap.md
8. docs/13-module-sdk/country-localization-regional-ops-readiness.md
9. docs/12-api/country-localization-regional-ops-api-contract.md
10. docs/11-ui-ux/country-localization-regional-ops-ui-contract.md
11. docs/05-infrastructure/country-localization-regional-ops-guide.md
12. docs/16-quality/sprint-42-country-localization-regional-ops-review.md

Implement only:
- country/regional feature registration foundation
- country catalog metadata foundation
- currency metadata foundation
- timezone metadata foundation
- language catalog metadata foundation
- organization regional settings foundation
- date/number/currency format metadata foundation
- payment provider availability mapping placeholder
- SMS provider availability mapping placeholder
- translation key namespace placeholder where practical
- regional readiness checklist placeholder where practical
- permission and feature checks
- audit records for regional setting changes
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- hardcoded Ghana-only behavior
- tax calculation engine
- legal/regulatory advice engine
- automatic translation of user-generated content
- provider-specific SDK calls inside regional configuration
- country-specific business logic inside Platform Core
- cross-organization regional settings visibility
- unrelated Commerce/POS expansion

Recommended branch:
feature/country-localization-regional-ops-foundation

Recommended PR title:
feat: add country localization regional operations foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Localization/Payment/SMS boundaries followed
- country/configuration rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused regional operations foundation for country metadata, currencies, timezones, languages, organization regional settings, formatting rules, provider availability mappings, translation key placeholders, and regional readiness checks.

## Final Rule

Regional behavior must be configuration-driven, not hardcoded. Platform Core must stay country-agnostic while engines and domains read country, currency, language, and provider availability through shared configuration.
