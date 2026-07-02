# Sprint 45 Authentication Session Device Security Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 45.

Sprint 45 prepares the Authentication Session, Device, and Security Control foundation after user access lifecycle, identity, permission, audit, governance, monitoring, and organization lifecycle foundations have introduced secure access needs.

## Prompt

```text
You are implementing Sprint 45 for Universal Platform.

Your task is to implement the Authentication Session, Device, and Security Control foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/12-api/identity-api-contract.md
3. docs/08-security/authentication-context-flow.md
4. docs/08-security/authorization-decision-flow.md
5. docs/08-security/08-security-standards.md
6. docs/14-roadmap/sprint-45-auth-session-device-security-roadmap.md
7. docs/13-module-sdk/auth-session-device-security-readiness.md
8. docs/12-api/auth-session-device-security-api-contract.md
9. docs/11-ui-ux/auth-session-device-security-ui-contract.md
10. docs/05-infrastructure/auth-session-device-security-guide.md
11. docs/16-quality/sprint-45-auth-session-device-security-review.md

Implement only:
- auth security feature registration foundation
- user session metadata foundation
- device metadata foundation
- session revocation foundation
- MFA requirement placeholder where practical
- security event metadata foundation
- suspicious access review placeholder where practical
- token/access key lifecycle placeholder where practical
- login risk signal placeholder where practical
- notification trigger placeholder where practical
- permission and feature checks
- audit records for sensitive security actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- password reset provider implementation
- external identity provider integration
- biometric authentication implementation
- device fingerprinting SDK integration
- hidden user impersonation
- automatic account lockout without review rules
- cross-organization session visibility
- unrelated Commerce/POS expansion

Recommended branch:
feature/auth-session-device-security-foundation

Recommended PR title:
feat: add auth session device security foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Identity/Permission/Audit boundaries followed
- session/device/security rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused security foundation for sessions, devices, revocation, MFA placeholders, security events, suspicious access review placeholders, token/access key lifecycle placeholders, risk signals, notifications, and audit-safe security actions.

## Final Rule

Authentication session and device security must be user-scoped, organization-aware where relevant, permission-protected, auditable, and provider-agnostic. Security controls should protect access without bypassing Identity or Permission Engine boundaries.
