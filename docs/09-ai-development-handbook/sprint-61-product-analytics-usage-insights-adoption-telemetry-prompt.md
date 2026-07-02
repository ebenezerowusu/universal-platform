# Sprint 61 Product Analytics Usage Insights Adoption Telemetry Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 61.

Sprint 61 prepares the Product Analytics, Usage Insights, and Adoption Telemetry foundation after feedback intake, product communication, notification center, feature flags, reporting, audit, privacy, and module foundations have introduced the need to understand adoption and usage patterns safely.

## Prompt

```text
You are implementing Sprint 61 for Universal Platform.

Your task is to implement the Product Analytics, Usage Insights, and Adoption Telemetry foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/12-api/reporting-analytics-api-contract.md
3. docs/12-api/data-privacy-consent-governance-api-contract.md
4. docs/12-api/configuration-feature-flags-rules-api-contract.md
5. docs/12-api/feedback-feature-requests-roadmap-intake-api-contract.md
6. docs/14-roadmap/sprint-61-product-analytics-usage-insights-adoption-telemetry-roadmap.md
7. docs/13-module-sdk/product-analytics-usage-insights-adoption-telemetry-readiness.md
8. docs/12-api/product-analytics-usage-insights-adoption-telemetry-api-contract.md
9. docs/11-ui-ux/product-analytics-usage-insights-adoption-telemetry-ui-contract.md
10. docs/05-infrastructure/product-analytics-usage-insights-adoption-telemetry-guide.md
11. docs/16-quality/sprint-61-product-analytics-usage-insights-adoption-telemetry-review.md

Implement only:
- product analytics feature registration foundation
- usage event metadata foundation
- feature adoption metric placeholder foundation
- module engagement metric placeholder foundation
- onboarding funnel placeholder where practical
- activation/retention summary placeholder where practical
- feature flag experiment/adoption linkage placeholder where practical
- feedback/support/product communication linkage where practical
- privacy-safe aggregation rules where practical
- permission and feature checks
- audit records for analytics configuration and access actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- invasive user surveillance
- raw activity replay
- cross-organization user-level analytics visibility
- selling user behavior data
- bypass of privacy/consent rules
- AI-only product decisioning
- replacement of Reporting/Analytics Engine
- unrelated Commerce/POS expansion

Recommended branch:
feature/product-analytics-usage-insights-adoption-telemetry-foundation

Recommended PR title:
feat: add product analytics usage insights adoption telemetry foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Analytics/Privacy/Reporting/Feature Flag boundaries followed
- aggregation and adoption insight rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused product analytics foundation for usage event metadata, feature adoption metrics, module engagement metrics, onboarding funnels, activation/retention summaries, feature flag adoption links, feedback/support/release communication links, privacy-safe aggregation rules, and audit-safe analytics access.

## Final Rule

Product analytics must help the platform learn from aggregated usage without becoming surveillance. Analytics must be privacy-aware, permission-protected, tenant-safe, and clearly separated from raw audit logs.
