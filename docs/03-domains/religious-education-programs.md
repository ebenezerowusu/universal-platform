# Religious Education Programs Specification

## Purpose

This document defines religious education and study program behavior inside the Religious Domain.

This module supports Bible study, discipleship, leadership classes, new convert classes, Islamic studies, training groups, and other structured learning programs.

## Scope

Education programs may include:

- Program creation
- Program groups
- Nested group tree
- Leaders and assistant leaders
- Participant assignment
- Sessions
- Attendance
- Completion tracking
- Reports

## Program Structure

Recommended structure:

```text
Program
  -> Program Group
    -> Subgroup
      -> Attendance Session
        -> Attendance Records
```

A program group tree can support unlimited nesting where useful.

## Examples

Possible programs:

- Bible Study
- New Converts Class
- Discipleship Program
- Leadership Training
- Youth Teaching Program
- Islamic Studies
- Custom Study Program

## Suggested Entities

Candidate entities:

- religious_programs
- religious_program_groups
- religious_program_group_members
- religious_program_group_leaders
- religious_program_sessions
- religious_program_completion_records

Attendance records may use the Religious Attendance module pattern.

## Group Tree Rules

Program groups should support:

- parent group
- path/depth fields where useful
- leader assignment
- member assignment
- reporting rollups

## Required Platform Engines

This module must use:

- Organization Engine for tenant context
- Permission Engine for access control
- Audit Engine for leadership and membership changes
- Notification Engine for reminders
- Reporting Engine for program reports
- Timeline Engine for participant history

## Permission Examples

- religious.education.programs.view
- religious.education.programs.manage
- religious.education.groups.manage
- religious.education.members.assign
- religious.education.attendance.mark
- religious.education.reports.view

## Events Published

- ReligiousProgramCreated
- ReligiousProgramGroupCreated
- ReligiousProgramMemberAssigned
- ReligiousProgramLeaderAssigned
- ReligiousProgramSessionCreated
- ReligiousProgramCompleted

## Reports

Reports may include:

- Program attendance
- Group attendance tree
- Leader submission status
- Participant completion
- Group growth
- Branch participation

## Security Requirements

- Program records are organization-owned.
- Leaders may only manage assigned groups unless granted wider permissions.
- Reports must respect permission and scope.

## Testing Requirements

Tests must cover:

- Create program
- Create group tree
- Assign member to group
- Assign leader
- Mark program attendance
- Completion tracking
- Permission checks
- Organization boundary checks

## Final Rule

Religious education is a domain module. It should reuse attendance, notification, reporting, and permission capabilities instead of duplicating platform logic.
