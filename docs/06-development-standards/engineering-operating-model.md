# Engineering Operating Model

## Purpose

This document defines how engineering work should operate for Universal Platform.

The goal is to keep development disciplined as the platform grows across backend, Flutter, domains, engines, and infrastructure.

## Operating Principle

Small, well-reviewed changes are preferred over large unclear changes.

Architecture must remain visible in daily engineering work.

## Work Intake

Every implementation task should define:

- goal
- layer
- scope
- out of scope
- required reading
- acceptance criteria
- data impact
- API impact
- test expectations

## Planning

Before coding, decide:

- which sprint or phase the task belongs to
- which module or engine owns it
- whether an ADR is needed
- whether API docs need updates
- whether migrations are required

## Implementation

During implementation:

- keep changes focused
- follow crate boundaries
- use application services
- use repositories for data access
- avoid provider logic in domains
- avoid domain logic in Platform Core

## Review

Every review should check:

- architecture boundaries
- data ownership
- API contract alignment
- permission and module effects
- tests
- documentation updates

## Release

Before release:

- confirm tests pass
- review migrations
- review runtime configuration
- prepare release notes
- confirm rollback or recovery direction

## Documentation

Documentation should change with implementation when behavior changes.

Do not let docs become historical notes that no longer match the system.

## Final Rule

Engineering process is part of architecture. A scalable platform needs disciplined execution, not only good design.
