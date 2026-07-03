# Sprint 62 Experimentation Controlled Rollouts Product Validation Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 62.

Sprint 62 prepares the Experimentation, Controlled Rollouts, and Product Validation foundation after product analytics, feature flags, feedback intake, product communication, monitoring, privacy, reporting, and support foundations have introduced safe validation and rollout decision needs.

## Prompt

```text
You are implementing Sprint 62 for Universal Platform.

Your task is to implement the Experimentation, Controlled Rollouts, and Product Validation foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/12-api/configuration-feature-flags-rules-api-contract.md
3. docs/12-api/product-analytics-usage-insights-adoption-telemetry-api-contract.md
4. docs/12-api/feedback-feature-requests-roadmap-intake-api-contract.md
5. docs/12-api/product-announcements-release-notes-change-communication-api-contract.md
6. docs/14-roadmap/sprint-62-experimentation-controlled-rollouts-product-validation-roadmap.md
7. docs/13-module-sdk/experimentation-controlled-rollouts-product-validation-readiness.md
8. docs/12-api/experimentation-controlled-rollouts-product-validation-api-contract.md
9. docs/11-ui-ux/experimentation-controlled-rollouts-product-validation-ui-contract.md
10. docs/05-infrastructure/experimentation-controlled-rollouts-product-validation-guide.md
11. docs/16-quality/sprint-62-experimentation-controlled-rollouts-product-validation-review.md

Implement only:
- experimentation feature registration foundation
- experiment metadata foundation
- experiment variant metadata placeholder foundation
- rollout cohort/segment placeholder foundation
- success metric linkage placeholder where practical
- guardrail metric linkage placeholder where practical
- experiment decision review placeholder foundation
- feature flag/configuration linkage where practical
- analytics/feedback/support linkage where practical
- product communication linkage placeholder where practical
- permission and feature checks
- audit records for experiment and rollout actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- automatic product decisioning
- user manipulation or dark-pattern experimentation
- medical/financial/legal eligibility experiments
- raw user behavior replay
- cross-organization user-level experiment visibility
- bypass of privacy/consent rules
- replacement of Feature Flag or Product Analytics engines
- unrelated Commerce/POS expansion

Recommended branch:
feature/experimentation-controlled-rollouts-product-validation-foundation

Recommended PR title:
feat: add experimentation controlled rollouts product validation foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Experimentation/Feature Flag/Analytics/Privacy boundaries followed
- rollout cohort and decision review rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused experimentation and validation foundation for experiment metadata, variants, rollout cohorts, success metrics, guardrails, decision reviews, feature/config links, analytics/feedback/support links, product communication links, and audit-safe experiment operations.

## Final Rule

Experiments and controlled rollouts must support careful product learning without automatic decisions, hidden manipulation, or privacy-invasive tracking. Feature Flags control rollout state; Product Analytics measures aggregate outcomes; Experimentation records validation design and decision review.
