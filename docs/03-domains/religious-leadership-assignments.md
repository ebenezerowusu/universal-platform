# Religious Leadership Assignments Specification

## Purpose

This document defines leadership assignment behavior inside the Religious Domain.

Leadership is domain-specific. The Platform Core provides users, roles, and permissions, but Religious leadership assignments describe responsibility over branches, congregations, services, groups, programs, and care areas.

## Scope

Leadership management may include:

- Assign leader
- Assign assistant leader
- Temporary or acting assignment
- Leadership start/end dates
- Responsibility scope
- Leadership history
- Approval workflow where needed
- Reporting line support

## Assignment Targets

A leader may be assigned to:

- branch
- congregation
- service
- group
- education program
- program group
- care team
- department
- event

## Relationship to Permissions

Leadership assignment is not the same as permission.

A leader assignment describes responsibility. Permission Engine controls what actions the user can perform.

Example:

A group leader may be responsible for a group, but they still need the required permission to mark attendance or send messages.

## Suggested Entities

Candidate entities:

- religious_leadership_assignments
- religious_leadership_assignment_history
- religious_leadership_roles
- religious_leadership_approval_requests

## Required Platform Engines

This module must use:

- Organization Engine for tenant context
- Permission Engine for authorization
- Workflow Engine for approvals where needed
- Audit Engine for assignment changes
- Notification Engine for assignment alerts
- Timeline Engine for leadership history

## Permission Examples

- religious.leadership.view
- religious.leadership.assign
- religious.leadership.remove
- religious.leadership.view_history
- religious.leadership.approve_assignment

## Events Published

- ReligiousLeaderAssigned
- ReligiousLeaderRemoved
- ReligiousLeadershipAssignmentUpdated
- ReligiousLeadershipAssignmentApproved

## Security Requirements

- Leadership changes should be audited.
- Parent organizations should only assign leaders to child structures if governance settings allow it.
- Sensitive leadership history should be permission-controlled.

## Testing Requirements

Tests must cover:

- Assign leader
- Remove leader
- Temporary assignment
- Assignment history
- Permission checks
- Organization boundary checks
- Governance checks where relevant

## Final Rule

Leadership assignment defines responsibility. Permission Engine defines authority. Both must work together.
