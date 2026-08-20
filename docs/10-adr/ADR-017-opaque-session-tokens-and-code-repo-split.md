# ADR-017: Opaque Session Tokens, Argon2id Hashing, and Code Repository Split

## Status

Accepted

## Date

2026-08-20

## Context

Implementation of the Sprint 2 identity foundation required decisions the
architecture documentation deliberately left open:

- The token/session strategy was undecided (`identity-engine.md`: "The exact
  token/session strategy should be finalized before implementation";
  `08-security-standards.md` treats JWT as conditional).
- The password hashing algorithm was never named ("approved server-side
  method").
- Two divergent login response shapes existed (`identity-api-contract.md`
  nested session vs `auth-organization-api-contracts.md` flat tokens).
- The project owner decided to split code out of this repository: this
  repository is documentation only, and implementation restarted fresh in
  dedicated repositories.

## Decision

```text
1. This repository (universal-platform) is documentation only. Code lives in:
   - universal-platform-backend (Rust/Axum backend)
   - universal-platform-app (Flutter client; scaffold lands at Sprint 6)
   The Sprint 1-3 code previously merged/branched in this repository is
   abandoned reference material.

2. Authentication uses opaque server-side session tokens, not JWT:
   - 32 random bytes, base64url-encoded, returned once to the client
   - only the SHA-256 hex digest is stored (user_sessions.token_hash)
   - sessions carry expires_at and revoked_at; logout revokes server-side
   - lifetime configured via SESSION_TTL_HOURS (default 168 hours)
   - refresh tokens are deferred; the identity contract permits this

3. Passwords are hashed with Argon2id (argon2 crate defaults), executed on a
   blocking thread. Minimum password length: 8 characters.

4. The login response follows identity-api-contract.md:
   data.user{id,displayName,email} + data.session{accessToken,expiresAt}.
   The flat accessToken/refreshToken shape in
   auth-organization-api-contracts.md is superseded.

5. Backend crate naming follows the platform- prefix (platform-api,
   platform-shared, platform-core, platform-engines, platform-domains,
   platform-infra) at the repository root; "core" is a reserved Cargo
   package name, and ADR-016 set the prefix precedent.

6. Database table naming follows the documentation (users, user_credentials,
   user_sessions, organizations, organization_users, organization_settings),
   not the abandoned branches' identity_users/organization_memberships.

7. Migrations run only as an explicit command
   (cargo run -p platform-api --bin migrate), never on application startup.
```

## Options Considered

### JWT access tokens

Pros: stateless verification, no session lookup per request.
Cons: revocation requires a denylist anyway; authorization claims go stale
(the security docs forbid trusting authorization from token claims); key
rotation complexity. Rejected for MVP — server-side sessions give immediate
revocation and match the security docs' lean.

### Porting the abandoned branch code

Pros: reuses ~2 sprints of scaffolding.
Cons: table naming conflicted with the documentation, no repositories/login
existed, branches were behind main. Rejected: fresh start with doc-aligned
naming (project owner's decision).

## Consequences

Positive: immediate session revocation; a database leak exposes no usable
tokens; doc-aligned schema removes the naming drift; the docs repo stays the
single source of architecture truth.

Tradeoffs: every authenticated request costs a session lookup (indexed by
token_hash); a refresh flow must be added before short-lived sessions are
practical; contributors must know code lives in separate repositories.

## Architecture Boundaries

- Platform Core remains domain-agnostic: yes
- Tenant isolation preserved: yes (membership-scoped organization queries)
- Permission checks not bypassed: permission engine arrives in Sprint 3
- Audit requirements preserved: audit engine arrives in Sprint 3
- Provider logic stays behind adapters: yes
- Configuration remains environment/tenant driven: yes (SESSION_TTL_HOURS)

## Implementation Impact

- universal-platform-backend PR #1: migrations 0001/0002, identity and
  organization modules, auth routes, tenant-scoped organization routes
- docs/12-api/auth-organization-api-contracts.md login shape is superseded
  by this ADR (identity-api-contract.md shape is canonical)

## Final Rule

Sessions are server-side and revocable. Documentation names win over
abandoned implementation names.
