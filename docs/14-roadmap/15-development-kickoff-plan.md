# Development Kickoff Plan

## Purpose

This document defines the first development sequence after foundation documentation.

The goal is to start implementation without violating the Platform Constitution.

## Main Rule

Begin with the Platform Core foundation before any product domain.

Do not start with Religious Domain screens, POS features, payment provider work, or mobile UI.

## Required Reading

Before coding, read:

- `CLAUDE.md`
- `docs/00-vision/00-platform-vision.md`
- `docs/01-platform-constitution/01-platform-constitution.md`
- `docs/02-architecture/02-platform-architecture.md`
- `docs/06-development-standards/06-backend-implementation-sequence.md`
- `docs/07-database/07-database-strategy.md`
- `docs/07-database/07-multi-tenancy-data-rules.md`
- `docs/08-security/08-security-standards.md`
- `docs/09-ai-development-handbook/09-ai-coding-agent-rules.md`

## First Implementation Order

1. Backend workspace structure
2. Runtime foundation
3. Configuration foundation
4. Health endpoint
5. Standard API response and error format
6. PostgreSQL connection
7. Migration setup
8. Identity Engine foundation
9. Organization Engine foundation
10. Permission Engine foundation
11. Audit Engine foundation
12. Module Registry foundation
13. Subscription Engine foundation
14. Payment, SMS, Storage, Notification skeletons
15. First Religious Domain preparation

## First Development Goal

The first development milestone should produce a running backend foundation, not a product feature.

Expected result:

- Backend project starts
- Health endpoint responds
- Configuration loads
- Database connection works
- Migrations are ready
- Shared API response format exists
- Basic tests exist

## What Not To Do First

Do not implement member management first.

Do not build Flutter screens first.

Do not connect external payment or SMS providers first.

Do not place domain logic in the Platform Core.

## Final Rule

Core first. Engines second. Domains third. UI after stable APIs.
