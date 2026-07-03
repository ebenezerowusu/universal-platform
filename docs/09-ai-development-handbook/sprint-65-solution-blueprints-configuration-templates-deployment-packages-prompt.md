# Sprint 65 Solution Blueprints Configuration Templates Deployment Packages Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 65.

Sprint 65 prepares the Solution Blueprints, Configuration Templates, and Deployment Packages foundation after implementation delivery, configuration/feature flags, workflow automation, module registry, localization, permissions, reporting, knowledge base, and customer success foundations have introduced repeatable setup and packaging needs.

## Prompt

```text
You are implementing Sprint 65 for Universal Platform.

Your task is to implement the Solution Blueprints, Configuration Templates, and Deployment Packages foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/12-api/configuration-feature-flags-rules-api-contract.md
3. docs/12-api/module-registry-subscription-api-contract.md
4. docs/12-api/workflow-automation-api-contract.md
5. docs/12-api/implementation-projects-onboarding-delivery-migration-runbooks-api-contract.md
6. docs/14-roadmap/sprint-65-solution-blueprints-configuration-templates-deployment-packages-roadmap.md
7. docs/13-module-sdk/solution-blueprints-configuration-templates-deployment-packages-readiness.md
8. docs/12-api/solution-blueprints-configuration-templates-deployment-packages-api-contract.md
9. docs/11-ui-ux/solution-blueprints-configuration-templates-deployment-packages-ui-contract.md
10. docs/05-infrastructure/solution-blueprints-configuration-templates-deployment-packages-guide.md
11. docs/16-quality/sprint-65-solution-blueprints-configuration-templates-deployment-packages-review.md

Implement only:
- solution blueprint feature registration foundation
- blueprint metadata foundation
- blueprint version placeholder foundation
- configuration template metadata foundation
- deployment package metadata foundation
- package component manifest placeholder foundation
- compatibility/preflight validation placeholder where practical
- preview/apply plan placeholder where practical
- implementation delivery/customer success linkage where practical
- rollback/change summary placeholder where practical
- permission and feature checks
- audit records for blueprint, template, and deployment package actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- direct production configuration mutation without preview/review
- hardcoded domain behavior inside Core
- unrestricted template installation across organizations
- destructive rollback automation
- bypass of permissions, module entitlements, or feature flags
- arbitrary script execution from packages
- marketplace monetization engine
- unrelated Commerce/POS expansion

Recommended branch:
feature/solution-blueprints-configuration-templates-deployment-packages-foundation

Recommended PR title:
feat: add solution blueprints configuration templates deployment packages foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Blueprint/Configuration/Module Registry/Implementation Delivery boundaries followed
- package preview, compatibility, and deployment safety rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused solution blueprint foundation for reusable blueprint metadata, blueprint versions, configuration templates, deployment packages, component manifests, compatibility/preflight validation, preview/apply plans, implementation/customer success links, rollback/change summaries, and audit-safe package operations.

## Final Rule

Solution blueprints should make onboarding repeatable without hardcoding domains into Core. Packages may describe configuration and setup intent, but applying changes must be permission-protected, entitlement-aware, previewed, auditable, and reversible by reviewed process only.
