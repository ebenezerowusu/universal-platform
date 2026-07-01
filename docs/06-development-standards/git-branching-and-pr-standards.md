# Git Branching and PR Standards

## Purpose

This document defines how work should be organized in Git for Universal Platform.

The goal is to keep changes reviewable, intentional, and safe.

## Branching Principle

Every meaningful change should happen on a branch.

The `main` branch should represent the accepted project foundation and stable implementation direction.

## Branch Naming

Recommended branch names:

```text
docs/<short-description>
feature/<short-description>
fix/<short-description>
refactor/<short-description>
chore/<short-description>
```

Examples:

```text
docs/payment-engine
feature/identity-foundation
fix/tenant-filter
chore/docker-compose
```

## Commit Messages

Commit messages should be short, clear, and action-oriented.

Examples:

```text
Add Payment Engine specification
Add identity foundation schema
Fix tenant filter in organization repository
```

## Pull Request Size

Prefer small, focused pull requests.

A pull request should usually address one concern:

- one document group
- one engine foundation
- one domain feature
- one bug fix
- one refactor

Avoid mixing unrelated changes.

## Pull Request Description

Every PR should explain:

- What changed
- Why it changed
- Architecture impact
- Tests run
- Risks or follow-up work

## Documentation Updates

If implementation changes architecture, standards, APIs, database structure, or module behavior, documentation must be updated in the same PR or a linked PR.

## AI Agent PRs

When Claude Code or another AI coding agent creates a PR, it must include:

- The docs it read
- The classification of the task
- Files changed
- Tests run
- Architecture rules followed
- Known limitations

## Main Branch Protection Direction

Before real development accelerates, the project should consider:

- PR reviews before merge
- CI checks before merge
- No direct pushes for implementation work
- Required status checks

## Final Rule

Small, documented, reviewable changes protect the platform architecture.
