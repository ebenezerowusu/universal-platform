# Definition of Done

## Purpose

This document defines the completion standard for work in Universal Platform.

A task is complete only when it is correct, reviewed, tested, and aligned with the architecture.

## General Completion Standard

Work is complete when:

- It addresses the intended scope.
- It follows the Platform Constitution.
- It is placed in the correct architecture layer.
- It respects organization boundaries where needed.
- It applies permission checks where needed.
- It has suitable tests.
- It follows API and database standards where applicable.
- It updates documentation where needed.
- It avoids unrelated changes.

## Backend Work

Backend work is complete when:

- Business rules are not placed inside route handlers.
- Organization context is handled correctly.
- Authorization is enforced where needed.
- Module and subscription checks are included where needed.
- Database migrations are included if needed.
- Error responses are structured and safe.
- Audit records are created for important actions.
- Success and failure cases are tested.

## Database Work

Database work is complete when:

- Migration files are included.
- Ownership and organization scope are clear.
- Foreign keys are used where appropriate.
- Indexes are considered.
- Existing data is protected.
- Naming follows project standards.

## API Work

API work is complete when:

- Routes follow API standards.
- Request validation exists.
- Authentication is applied where needed.
- Authorization is applied where needed.
- Response format is consistent.
- Error format is consistent.
- OpenAPI docs are updated where appropriate.

## Engine Work

Capability engine work is complete when:

- The interface is clear.
- Domain usage is clear.
- Provider or infrastructure details are abstracted.
- Organization context is respected.
- Tests cover core behavior.

## Domain Work

Domain work is complete when:

- It does not place domain rules inside Platform Core.
- It uses shared engines for shared capabilities.
- It publishes events where useful.
- It respects module installation and subscription access.
- It includes permissions.
- It includes organization boundary tests where needed.

## Documentation Work

Documentation work is complete when:

- The purpose is clear.
- Responsibilities and boundaries are clear.
- It does not contradict accepted ADRs.
- It supports future implementation decisions.

## Final Rule

Complete means architecturally aligned, tested, documented where needed, and safe to build on.
