# Sprint 5 Issue Breakdown

## Purpose

This document breaks Sprint 5 into implementation issues.

Sprint 5 should begin the Religious MVP after Core, Permission, Audit, Module Registry, and engine skeletons are usable.

## Issue 1: Register Religious MVP Modules

### Goal

Register the first Religious MVP modules.

### Scope

- religious.members
- religious.visitors
- religious.structure
- religious.attendance
- religious.reports

### Acceptance Criteria

- Modules are registered with stable keys.
- Modules can be added to an organization.
- Module availability can be checked by Religious use cases.

## Issue 2: Create Religious Settings Foundation

### Goal

Create the first Religious Domain settings records.

### Scope

- religious settings table
- settings repository
- settings service
- default settings seed where useful

### Acceptance Criteria

- Organization can have Religious settings.
- Settings remain domain-specific.
- Platform Core does not know Religious settings.

## Issue 3: Create Member Foundation

### Goal

Implement basic member management.

### Scope

- religious_members migration
- status history migration
- member repository
- create member use case
- list member use case
- get member detail use case
- update member use case

### Acceptance Criteria

- Members can be created.
- Members can be listed by selected organization context.
- Member status history can be recorded.
- Member routes use permission and module checks.

## Issue 4: Create Structure Foundation

### Goal

Implement basic congregations, services, and groups.

### Scope

- congregations table
- services table
- groups table
- group members table
- repositories
- create/list use cases

### Acceptance Criteria

- Congregations can be created and listed.
- Services can be created and listed.
- Groups can be created and listed.
- Group membership can be represented.

## Issue 5: Create Visitor Foundation

### Goal

Implement basic visitor management.

### Scope

- visitors table
- visitor follow-up table
- visitor repository
- create visitor use case
- list visitor use case
- convert visitor use case

### Acceptance Criteria

- Visitors can be created.
- Visitors can be listed.
- Visitor can be converted to member through application service.
- Conversion creates expected member record.

## Issue 6: Create Attendance Foundation

### Goal

Implement manual attendance foundation.

### Scope

- attendance sessions table
- attendance records table
- attendance repository
- create session use case
- mark attendance use case

### Acceptance Criteria

- Attendance session can be created.
- Attendance records can be marked.
- Duplicate records for same member and session are handled safely.
- Important edits can be audited later.

## Issue 7: Basic Religious Reports

### Goal

Create first Religious dashboard/report foundation.

### Scope

- member count
- visitor count
- group count
- attendance summary foundation

### Acceptance Criteria

- Basic reports return organization-scoped data.
- Reports follow Reporting Engine direction.

## Issue 8: Sprint 5 Tests

### Goal

Cover Religious MVP foundation behavior.

### Scope

- create member
- list member
- create visitor
- convert visitor
- create congregation
- create service
- create group
- create attendance session
- mark attendance

### Acceptance Criteria

- Tests pass.
- Tests show Religious Domain uses Core and Engines correctly.

## Final Rule

Sprint 5 begins Religious Domain work, but it must prove the platform architecture instead of bypassing it.
