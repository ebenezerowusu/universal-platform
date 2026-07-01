# Religious Migration Plan

## Purpose

This document defines the recommended migration order for the Religious Domain.

Religious Domain migrations should only begin after Platform Core foundations are in place.

## Migration Principle

Religious migrations must depend on Platform Core organization ownership and shared engine foundations.

Every Religious Domain table should include organization scope unless there is a documented exception.

## Required Core Dependencies

Before Religious migrations begin, the platform should already have:

- users
- organizations
- organization_users
- permissions
- roles
- modules
- organization_modules
- subscription foundations
- audit logs
- storage foundation where file references are needed

## Recommended Migration Order

### 1. Religious Settings

Create:

- religious_settings
- religious_terminology_settings optional

Purpose:

- Store Religious Domain configuration per organization.

### 2. Structure Foundation

Create:

- religious_branch_relations
- religious_congregations
- religious_services
- religious_groups
- religious_group_members

Purpose:

- Establish branch, congregation, service, and group structure.

### 3. Members

Create:

- religious_members
- religious_member_status_history
- religious_member_documents
- religious_member_notes

Purpose:

- Manage member records and lifecycle.

### 4. Visitors

Create:

- religious_visitors
- religious_visitor_follow_ups
- religious_visitor_status_history

Purpose:

- Manage visitors and follow-up.

### 5. Attendance

Create:

- religious_attendance_sessions
- religious_attendance_records
- religious_attendance_edit_history

Purpose:

- Track attendance across targets.

### 6. Education Programs

Create:

- religious_programs
- religious_program_groups
- religious_program_group_members
- religious_program_group_leaders
- religious_program_completion_records

Purpose:

- Support study and training programs.

### 7. Family and Households

Create:

- religious_family_relationships
- religious_family_relationship_requests
- religious_households
- religious_household_members

Purpose:

- Support family and household relationships.

### 8. Giving and Pledges

Create:

- religious_giving_categories
- religious_giving_records
- religious_pledges
- religious_pledge_payments

Purpose:

- Support giving and pledge tracking.

### 9. Events

Create:

- religious_events
- religious_event_registrations
- religious_event_attendance
- religious_event_volunteers

Purpose:

- Support events and registrations.

### 10. Care Modules

Create:

- religious_prayer_requests
- religious_counseling_cases
- religious_counseling_appointments
- religious_welfare_requests
- religious_care_follow_ups

Purpose:

- Support care, prayer, counseling, and welfare.

### 11. Custom Forms

Create:

- religious_custom_field_definitions
- religious_custom_field_values
- religious_form_definitions
- religious_form_sections
- religious_form_fields

Purpose:

- Support organization-specific form flexibility.

## MVP Migration Subset

For the first Religious MVP, prioritize:

1. religious_settings
2. religious_congregations
3. religious_services
4. religious_groups
5. religious_members
6. religious_visitors
7. religious_attendance_sessions
8. religious_attendance_records

## Review Rules

Review each migration for:

- organization_id presence
- foreign keys
- indexes
- scoped uniqueness
- soft delete needs
- audit requirements
- report query needs

## Final Rule

Religious migrations must be layered on Platform Core and must not recreate shared platform capabilities.
