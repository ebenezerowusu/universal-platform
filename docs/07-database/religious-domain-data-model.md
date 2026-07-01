# Religious Domain Data Model

## Purpose

This document defines the Religious Domain data model at a planning level.

The Religious Domain must sit on top of Platform Core. It must not redefine identity, organizations, permissions, payments, SMS, storage, or audit.

## Data Model Principle

Every Religious Domain table must be organization-owned unless there is a documented exception.

Most tables should include:

```text
id
organization_id
created_at
updated_at
deleted_at nullable
```

## Member Tables

### religious_members

Purpose:

- Store religious member profiles.

Candidate columns:

- id
- organization_id
- linked_user_id nullable
- member_code nullable
- first_name
- last_name
- other_names nullable
- gender nullable
- date_of_birth nullable
- phone nullable
- email nullable
- photo_file_id nullable
- status
- joined_at nullable
- branch_id nullable
- congregation_id nullable
- created_at
- updated_at
- deleted_at

### religious_member_status_history

Purpose:

- Track member status changes.

Candidate columns:

- id
- organization_id
- member_id
- old_status
- new_status
- reason nullable
- changed_by
- changed_at

## Visitor Tables

### religious_visitors

Purpose:

- Store visitor records.

Candidate columns:

- id
- organization_id
- first_name
- last_name
- phone nullable
- email nullable
- invited_by_member_id nullable
- status
- first_visit_at nullable
- branch_id nullable
- created_at
- updated_at

### religious_visitor_follow_ups

Purpose:

- Track visitor follow-up work.

Candidate columns:

- id
- organization_id
- visitor_id
- assigned_to_user_id nullable
- status
- due_at nullable
- notes nullable
- completed_at nullable
- created_at
- updated_at

## Structure Tables

### religious_congregations

Purpose:

- Store congregations within an organization or branch.

Candidate columns:

- id
- organization_id
- branch_id nullable
- name
- description nullable
- status
- created_at
- updated_at

### religious_services

Purpose:

- Store service definitions.

Candidate columns:

- id
- organization_id
- congregation_id
- name
- day_of_week nullable
- start_time nullable
- end_time nullable
- venue nullable
- status
- created_at
- updated_at

### religious_groups

Purpose:

- Store groups, departments, cells, study groups, teams, or classes.

Candidate columns:

- id
- organization_id
- congregation_id nullable
- parent_group_id nullable
- group_type
- name
- description nullable
- status
- created_at
- updated_at

### religious_group_members

Purpose:

- Link members to groups.

Candidate columns:

- id
- organization_id
- group_id
- member_id
- status
- joined_at nullable
- created_at
- updated_at

## Attendance Tables

### religious_attendance_sessions

Purpose:

- Define attendance sessions.

Candidate columns:

- id
- organization_id
- target_type
- target_id
- title
- session_date
- start_time nullable
- end_time nullable
- status
- created_by
- created_at
- updated_at

### religious_attendance_records

Purpose:

- Store member attendance status.

Candidate columns:

- id
- organization_id
- session_id
- member_id
- status
- method
- marked_by nullable
- marked_at nullable
- metadata nullable
- created_at
- updated_at

## Family and Household Tables

### religious_family_relationships

Purpose:

- Store accepted family relationships.

Candidate columns:

- id
- organization_id
- member_id
- related_member_id
- relationship_type
- status
- created_at
- updated_at

### religious_households

Purpose:

- Store confirmed households.

Candidate columns:

- id
- organization_id
- name nullable
- primary_member_id nullable
- status
- created_at
- updated_at

### religious_household_members

Purpose:

- Link members to households.

Candidate columns:

- id
- organization_id
- household_id
- member_id
- role nullable
- status
- created_at
- updated_at

## Giving Tables

### religious_giving_categories

Purpose:

- Define giving categories.

Candidate columns:

- id
- organization_id
- name
- code
- status
- created_at
- updated_at

### religious_giving_records

Purpose:

- Store giving records linked to verified payments or manual entries.

Candidate columns:

- id
- organization_id
- member_id nullable
- category_id
- amount
- currency
- payment_transaction_id nullable
- giving_date
- status
- created_at
- updated_at

### religious_pledges

Purpose:

- Track pledge commitments.

Candidate columns:

- id
- organization_id
- member_id nullable
- title
- target_amount
- currency
- start_date nullable
- end_date nullable
- status
- created_at
- updated_at

## Events and Care Tables

Additional tables may include:

- religious_events
- religious_event_registrations
- religious_prayer_requests
- religious_counseling_cases
- religious_welfare_requests
- religious_care_follow_ups

These should follow the same organization ownership, permission, audit, and reporting rules.

## Final Rule

Religious Domain tables model religious business concepts only. Shared platform capabilities must remain in engines and Platform Core.
