# Religious Congregations, Services, and Groups Specification

## Purpose

This document defines congregations, services, and groups inside the Religious Domain.

These structures help organizations organize people, gatherings, functional teams, cells, departments, and study groups.

## Core Distinction

Use this distinction:

```text
Congregation = who the people are grouped as
Service = when they gather
Group = smaller functional/community/study unit inside a congregation
```

## Structure

Recommended structure:

```text
Organization
  -> Branch
    -> Congregation
      -> Service
      -> Group
```

## Congregations

Congregations may represent large people groups such as:

- Adult congregation
- Youth congregation
- Teens congregation
- Children congregation
- Language-based congregation
- Custom congregation

## Services

Services represent scheduled gatherings.

Examples:

- First service
- Second service
- Midweek service
- Prayer service
- Children service
- Special service

Service fields may include:

- congregation_id
- name
- day_of_week
- start_time
- end_time
- venue
- status

## Groups

Groups live inside congregations.

Group types may include:

- department
- cell
- study_group
- fellowship
- ministry
- class
- team

Examples:

- Choir
- Ushers
- Media
- Prayer Team
- Home Cell
- Bible Study Group
- Leadership Class

## Suggested Entities

Candidate entities:

- religious_congregations
- religious_services
- religious_groups
- religious_group_members
- religious_group_leaders
- religious_service_schedules

## Required Platform Engines

This module must use:

- Organization Engine for tenant context
- Permission Engine for access control
- Audit Engine for leadership and structural changes
- Notification Engine for announcements
- Reporting Engine for structure reports
- Localization Engine for labels

## Permission Examples

- religious.congregations.view
- religious.congregations.manage
- religious.services.view
- religious.services.manage
- religious.groups.view
- religious.groups.manage
- religious.groups.manage_members
- religious.groups.assign_leaders

## Events Published

- ReligiousCongregationCreated
- ReligiousServiceCreated
- ReligiousGroupCreated
- ReligiousGroupMemberAdded
- ReligiousGroupLeaderAssigned

## Reports

Reports may include:

- Members by congregation
- Groups by congregation
- Service attendance summary
- Group member counts
- Leader assignment coverage

## Security Requirements

- All records are organization-owned.
- Leaders may only manage assigned groups unless granted wider permissions.
- Structural changes should be audited where important.

## Testing Requirements

Tests must cover:

- Create congregation
- Create service under congregation
- Create group under congregation
- Add member to group
- Assign leader
- Permission checks
- Organization boundary checks

## Final Rule

Congregations, services, and groups are Religious Domain structures. The Platform Core must not contain these business concepts.
