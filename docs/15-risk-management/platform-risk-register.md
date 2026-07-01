# Platform Risk Register

## Purpose

This document tracks major risks for Universal Platform.

The goal is to identify risks early and document mitigation direction before implementation grows.

## Risk 1: Platform Core Becomes Domain-Specific

### Description

Core may accidentally start containing Religious, Commerce, POS, HR, or Finance business concepts.

### Impact

High.

### Mitigation

- Enforce Platform Constitution.
- Review all Core changes.
- Keep domain language inside domains.
- Use ADRs for boundary changes.

## Risk 2: Provider Logic Leaks Into Domains

### Description

Domains may call payment, SMS, storage, or other providers directly.

### Impact

High.

### Mitigation

- Use capability engines and provider adapters.
- Review provider-related code carefully.
- Require provider abstractions before integrations.

## Risk 3: Tenant Boundaries Are Applied Inconsistently

### Description

Organization-owned records may not consistently use organization context.

### Impact

High.

### Mitigation

- Use scoped repository methods.
- Add scoped query tests.
- Review all organization-owned migrations.

## Risk 4: MVP Scope Expands Too Quickly

### Description

The project may try to build too many domains before the platform foundation is stable.

### Impact

High.

### Mitigation

- Follow sprint execution order.
- Keep first domain MVP small.
- Defer advanced features.

## Risk 5: Flutter UI Outruns Backend Contracts

### Description

Flutter screens may be built before backend API contracts stabilize.

### Impact

Medium.

### Mitigation

- Use API contract-first delivery.
- Use placeholders or mock data only where contracts are not ready.
- Keep Flutter navigation backend-informed.

## Risk 6: Local Development Becomes Difficult

### Description

Developers may struggle to run the platform locally.

### Impact

Medium.

### Mitigation

- Maintain local development docs.
- Use Docker Compose for services where practical.
- Provide example environment files.

## Risk 7: Background Jobs Are Hard to Debug

### Description

SMS, reports, imports, and provider follow-up jobs may fail without clear visibility.

### Impact

Medium.

### Mitigation

- Use job contracts.
- Track job status.
- Include trace IDs and error codes.

## Risk 8: Documentation and Code Drift Apart

### Description

Architecture docs may become outdated after implementation begins.

### Impact

Medium.

### Mitigation

- Update docs in the same PR as behavior changes.
- Use PR template documentation checklist.
- Review API and migration docs during code review.

## Final Rule

Risks should be reviewed during planning and after major implementation milestones.
