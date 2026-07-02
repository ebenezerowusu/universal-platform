# Sprint 48 Configuration Feature Flags Rules Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 48.

Sprint 48 prepares the Configuration, Feature Flags, and Runtime Rules foundation after async operations, regional settings, module registry, billing entitlements, notifications, workflow, and domain foundations have introduced configuration-driven behavior needs.

## Prompt

```text
You are implementing Sprint 48 for Universal Platform.

Your task is to implement the Configuration, Feature Flags, and Runtime Rules foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/01-platform-constitution/01-platform-constitution.md
3. docs/12-api/module-registry-api-contract.md
4. docs/12-api/subscription-feature-api-contract.md
5. docs/12-api/country-localization-regional-ops-api-contract.md
6. docs/14-roadmap/sprint-48-configuration-feature-flags-rules-roadmap.md
7. docs/13-module-sdk/configuration-feature-flags-rules-readiness.md
8. docs/12-api/configuration-feature-flags-rules-api-contract.md
9. docs/11-ui-ux/configuration-feature-flags-rules-ui-contract.md
10. docs/05-infrastructure/configuration-feature-flags-rules-guide.md
11. docs/16-quality/sprint-48-configuration-feature-flags-rules-review.md

Implement only:
- configuration feature registration foundation
- configuration namespace metadata foundation
- configuration key/value metadata foundation
- tenant override foundation
- feature flag metadata foundation
- rollout rule placeholder foundation
- runtime rule definition placeholder where practical
- rule evaluation log placeholder where practical
- default/fallback value foundation
- configuration change history foundation
- permission and feature checks
- audit records for sensitive configuration changes
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- arbitrary code execution rules
- unsafe scriptable rule engine
- direct provider-specific configuration inside domains
- bypass of subscription/feature entitlement checks
- cross-organization configuration visibility
- secrets management vault
- tax/legal rules engine
- unrelated Commerce/POS expansion

Recommended branch:
feature/configuration-feature-flags-rules-foundation

Recommended PR title:
feat: add configuration feature flags rules foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Configuration/Entitlement/Audit boundaries followed
- tenant override and rollout rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused configuration foundation for namespaces, keys, default values, tenant overrides, feature flags, rollout placeholders, runtime rule placeholders, evaluation logs, change history, and audit-safe configuration operations.

## Final Rule

Configuration must be tenant-aware, permission-protected, auditable, default-safe, and entitlement-aware. Platform behavior should be configured through shared configuration boundaries instead of hardcoded inside domains.
