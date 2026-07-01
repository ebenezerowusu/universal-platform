# Religious Care, Prayer, Counseling, and Welfare Specification

## Purpose

This document defines care-related modules inside the Religious Domain.

These modules support pastoral care, prayer support, counseling coordination, welfare assistance, visitation, and follow-up.

## Scope

Care-related features may include:

- Prayer requests
- Counseling requests
- Counseling appointments
- Welfare requests
- Care cases
- Hospital visitation records
- Bereavement support records
- Follow-up tasks
- Confidential notes
- Care reports

## Care Areas

### Prayer Requests

Features:

- Submit prayer request
- Set visibility level
- Assign to prayer team or leader
- Track status
- Record testimony or outcome where appropriate

### Counseling

Features:

- Counseling request
- Counselor assignment
- Appointment scheduling
- Confidential notes
- Follow-up tasks
- Case status

### Welfare

Features:

- Welfare request
- Support category
- Review/approval process
- Assistance record
- Household linkage where useful
- Follow-up notes

### Visitation and Care

Features:

- Visitation record
- Care assignment
- Follow-up schedule
- Care outcome

## Visibility Levels

Possible visibility levels:

- public_to_leaders
- assigned_team_only
- confidential

Confidential records require strict permission checks.

## Suggested Entities

Candidate entities:

- religious_prayer_requests
- religious_counseling_cases
- religious_counseling_appointments
- religious_welfare_requests
- religious_welfare_support_records
- religious_care_cases
- religious_care_follow_ups
- religious_confidential_notes

## Required Platform Engines

This module must use:

- Organization Engine for tenant context
- Permission Engine for access control
- Workflow Engine for review and follow-up processes
- Notification Engine for reminders
- Audit Engine for confidential access and sensitive changes
- Timeline Engine for care history where appropriate
- Storage Engine for related documents where needed
- Reporting Engine for aggregate reports

## Permission Examples

- religious.care.view
- religious.care.manage
- religious.prayer.view
- religious.prayer.manage
- religious.counseling.view
- religious.counseling.manage
- religious.counseling.view_confidential
- religious.welfare.view
- religious.welfare.manage
- religious.welfare.approve

## Events Published

- ReligiousPrayerRequestSubmitted
- ReligiousPrayerRequestAssigned
- ReligiousCounselingCaseCreated
- ReligiousCounselingAppointmentScheduled
- ReligiousWelfareRequestCreated
- ReligiousWelfareRequestReviewed
- ReligiousCareFollowUpCreated

## Reports

Reports should avoid exposing confidential details.

Possible aggregate reports:

- Prayer request count
- Counseling case count
- Welfare requests by status
- Care follow-up completion
- Assigned care tasks

## Security Requirements

- Confidential records require explicit permissions.
- Sensitive access should be audited.
- Reports should aggregate sensitive data where possible.
- Notes must not be exposed through general member profiles.
- Organization boundaries must be enforced.

## Testing Requirements

Tests must cover:

- Create prayer request
- Assign request
- Create counseling case
- Schedule appointment
- Create welfare request
- Permission checks for confidential records
- Audit record for sensitive access
- Organization boundary checks

## Final Rule

Care-related modules handle sensitive human information. Access, audit, and confidentiality must be designed from the beginning.
