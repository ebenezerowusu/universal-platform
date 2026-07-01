# Phase 0 Roadmap: Foundation Before Development

## Purpose

Phase 0 exists to create the architecture foundation before writing production code.

The goal is to make sure every engineer and AI coding agent understands the platform vision, boundaries, standards, and roadmap before implementation begins.

## Phase 0 Rule

No production feature development should begin until the foundation documentation is complete enough to guide implementation.

## Current Foundation Documents

Initial foundation documents:

- Platform Vision
- Platform Constitution
- Platform Architecture Overview
- Deployment Strategy
- Engineering Standards
- Database Strategy
- AI Coding Agent Rules
- ADR-001 Platform-First Architecture
- ADR-002 PostgreSQL Primary Database
- ADR-003 Rust Axum Backend

## Remaining Phase 0 Documents

### Domains

Create detailed documents for:

- Religious Domain
- Commerce Domain
- POS Module
- Finance Domain
- HR Domain
- Logistics/Future Domain Strategy

Each domain document should define:

- Purpose
- Scope
- Entities
- Workflows
- Permissions
- Events
- Reports
- APIs
- Configuration

### Capability Engines

Create detailed documents for:

- Identity Engine
- Organization Engine
- Permission Engine
- Module Registry
- Subscription Engine
- Payment Engine
- SMS Engine
- Notification Engine
- Workflow Engine
- Storage Engine
- Audit Engine
- Reporting Engine
- Localization Engine
- Timeline Engine
- Import/Export Engine

Each engine document should define:

- Purpose
- Responsibilities
- Interfaces
- Domain usage
- Provider adapters if applicable
- Events
- Security concerns
- Configuration

### API Standards

Define:

- URL patterns
- Versioning
- Authentication
- Authorization
- Pagination
- Filtering
- Sorting
- Error format
- Response format
- OpenAPI requirements

### Security Standards

Define:

- Authentication
- JWT/session strategy
- RBAC
- Tenant isolation
- Rate limiting
- Secrets handling
- Audit requirements
- Data privacy

### Module SDK

Define:

- Module manifest
- Module lifecycle
- Navigation registration
- Permission registration
- Event publishing and consumption
- Settings schema
- Migration handling
- Installation and upgrade process

### UI/UX Standards

Define:

- Flutter design system
- Theme structure
- Responsive rules
- Mobile vs desktop layout rules
- Localization
- Accessibility
- Module-aware navigation

### Infrastructure Details

Define:

- Docker Compose files
- VPS setup
- Nginx strategy
- SSL
- Backup scripts
- Monitoring
- Deployment scripts

## Recommended Implementation Start Point After Phase 0

After foundation documentation is ready, implementation should begin with:

1. Backend workspace setup
2. Configuration system
3. Health check endpoint
4. PostgreSQL connection
5. Migration tooling
6. Core identity models
7. Organization/tenant models
8. RBAC foundation
9. Module registry foundation
10. Audit logging foundation

Do not start with Religious features first.

Build Platform Core first, then install domains on top.

## Claude Code Instruction After Phase 0

When Claude Code starts development, the first prompt should be:

```text
Read the entire docs/ folder first.
Do not write code until you understand the Platform Constitution, Architecture Overview, Database Strategy, and AI Coding Agent Rules.
Start by creating the backend foundation according to the documented architecture.
Do not implement domain-specific features inside the Platform Core.
Explain your implementation plan before coding.
```

## Phase 0 Completion Criteria

Phase 0 is complete when:

- The vision is clear.
- The constitution is accepted.
- The architecture is documented.
- The database strategy is documented.
- The first domain is specified.
- The core engines are specified.
- The AI coding-agent rules are documented.
- The first ADRs are written.
- The development roadmap is clear.

## Final Reminder

A strong foundation saves years of refactoring.

Build slowly at the foundation stage so the platform can grow faster later.
