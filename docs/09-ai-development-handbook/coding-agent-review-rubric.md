# Coding Agent Review Rubric

## Purpose

This document defines how to review work produced by Claude Code or other coding agents.

The goal is to keep AI-generated work aligned with the platform architecture.

## Review Principle

AI-generated work must meet the same standard as human-generated work.

Do not merge work just because it appears complete.

## Required Agent Summary

Every agent-produced change should state:

- Task classification
- Documents read
- Files changed
- Architecture rules applied
- Tests run
- Risks and limitations
- Follow-up work

## Architecture Rubric

Check:

- Platform Core remains domain-agnostic.
- Domain logic stays in domains.
- Shared capabilities use engines.
- Provider details stay in adapters.
- API handlers stay thin.
- Infrastructure does not own business rules.

## Security Rubric

Check:

- Organization boundaries are enforced.
- Permissions are checked.
- Sensitive actions are audited.
- Secret values are not committed.
- Errors are safe.
- Provider responses are not leaked carelessly.

## Database Rubric

Check:

- Migrations are present where needed.
- Tables have organization scope where needed.
- Indexes are considered.
- Foreign keys are appropriate.
- Unique constraints are scoped correctly.
- Seed data is idempotent.

## Testing Rubric

Check:

- Tests exist at the correct level.
- Happy path is tested.
- Failure path is tested.
- Permission failure is tested.
- Organization boundary is tested where relevant.

## Scope Rubric

Check:

- The agent did not add unrelated features.
- The agent did not jump ahead of the roadmap.
- The agent did not create domain shortcuts.
- The agent did not silently change architecture decisions.

## Documentation Rubric

Check:

- Relevant docs were updated.
- New architecture decisions have ADRs where needed.
- API changes are documented.
- Database changes are documented.

## Final Rule

AI can accelerate development, but architecture review remains mandatory.
