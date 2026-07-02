# Sprint 49 Secure Credential Provider Connections Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 49.

Sprint 49 prepares the Secure Credential References, Provider Connections, and Environment Profiles foundation after configuration, payment operations, SMS provider readiness, storage, developer integrations, and regional provider availability have introduced sensitive provider connection needs.

## Prompt

```text
You are implementing Sprint 49 for Universal Platform.

Your task is to implement the Secure Credential References, Provider Connections, and Environment Profiles foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/08-security/08-security-standards.md
3. docs/12-api/configuration-feature-flags-rules-api-contract.md
4. docs/12-api/payment-operations-reconciliation-api-contract.md
5. docs/12-api/sms-provider-admin-api-contract.md
6. docs/14-roadmap/sprint-49-secure-credential-provider-connections-roadmap.md
7. docs/13-module-sdk/secure-credential-provider-connections-readiness.md
8. docs/12-api/secure-credential-provider-connections-api-contract.md
9. docs/11-ui-ux/secure-credential-provider-connections-ui-contract.md
10. docs/05-infrastructure/secure-credential-provider-connections-guide.md
11. docs/16-quality/sprint-49-secure-credential-provider-connections-review.md

Implement only:
- secure provider connection feature registration foundation
- credential reference metadata foundation
- provider connection metadata foundation
- environment profile metadata foundation
- credential rotation placeholder foundation
- provider connection health check placeholder where practical
- provider capability mapping linkage where practical
- credential access request placeholder where practical
- credential usage audit/event history foundation
- safe configuration linkage to Sprint 48 configuration records
- permission and feature checks
- audit records for sensitive credential/provider actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- exposing credential values in API responses
- storing provider credentials inside normal configuration values
- arbitrary provider SDK execution
- direct provider-specific logic inside domain modules
- unmanaged plaintext credential storage
- cross-organization credential visibility
- automatic credential rotation without approval rules
- unrelated Commerce/POS expansion

Recommended branch:
feature/secure-credential-provider-connections-foundation

Recommended PR title:
feat: add secure credential provider connections foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Credential/Provider/Configuration/Audit boundaries followed
- sensitive data handling rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused secure provider connection foundation for credential references, provider connection metadata, environment profiles, rotation placeholders, connection health placeholders, capability mapping linkage, access request placeholders, usage audit history, and safe configuration linkage.

## Final Rule

Sensitive provider access must be referenced, not exposed. Domains should use provider adapters and credential references through shared boundaries instead of reading or storing provider credentials directly.
