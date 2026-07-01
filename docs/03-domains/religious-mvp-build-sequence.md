# Religious MVP Build Sequence

## Purpose

This document defines the recommended build sequence for the first Religious Domain MVP.

The Religious Domain should start only after Platform Core foundations are usable.

## Prerequisites

Before Religious MVP work begins, the platform should have:

- Identity foundation
- Organization foundation
- Permission foundation
- Audit foundation
- Module Registry foundation
- Basic subscription or feature gate pattern
- Engine skeletons for storage, notification, reporting, payment, and SMS

## Build Sequence

### Step 1: Religious Module Registration

Register initial Religious modules:

- religious.members
- religious.visitors
- religious.structure
- religious.attendance
- religious.reports

Expected result:

- Modules can be installed and activated for an organization.

### Step 2: Religious Settings

Implement organization-level Religious settings.

Examples:

- terminology labels
- default branch behavior
- member code format later
- attendance settings later

### Step 3: Member Foundation

Implement:

- member table
- member create
- member list
- member detail
- member update
- status history

### Step 4: Structure Foundation

Implement:

- congregations
- services
- groups
- group membership

### Step 5: Visitor Foundation

Implement:

- visitor create
- visitor list
- follow-up foundation
- convert visitor to member

### Step 6: Attendance Foundation

Implement:

- attendance sessions
- manual attendance records
- duplicate prevention
- edit audit record

### Step 7: Basic Reports

Implement basic reports:

- member count
- visitor count
- attendance summary
- group count

### Step 8: Communication Hook

Connect basic announcement/reminder workflows to Notification Engine skeleton.

SMS provider integration can remain mocked or deferred.

## Out of Initial MVP

Defer:

- QR attendance
- GPS attendance
- advanced family linking
- full giving reports
- full care/counseling module
- advanced hierarchy governance
- offline sync

## Completion Standard

Religious MVP foundation is complete when:

- Religious modules install for an organization.
- Members can be managed.
- Visitors can be managed and converted.
- Congregations, services, and groups exist.
- Manual attendance works.
- Basic reports work.
- Permissions and organization scope are enforced.
- Sensitive changes are audited.

## Final Rule

Religious MVP must prove the platform model. It must not bypass Core, Engines, Module Registry, Permission Engine, or Audit Engine.
