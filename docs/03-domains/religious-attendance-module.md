# Religious Attendance Module Specification

## Purpose

This document defines attendance behavior inside the Religious Domain.

The Religious Domain should use the reusable Attendance capability pattern and must avoid building attendance logic that cannot be reused by other domains later.

## Scope

Religious attendance may apply to:

- Services
- Congregations
- Groups
- Cells
- Departments
- Religious education programs
- Events

## Attendance Methods

Supported methods may include:

- Manual marking
- Bulk marking
- QR check-in
- GPS/location check-in
- Mobile self check-in
- Staff-assisted check-in
- Offline marking with later sync

MVP may begin with manual attendance.

## Attendance Statuses

Possible statuses:

- present
- absent
- late
- excused
- pending_review

## Attendance Settings

Possible organization/module settings:

- allow_manual_marking
- allow_bulk_marking
- allow_qr_check_in
- allow_location_check_in
- allow_mobile_self_check_in
- require_location_for_self_check_in
- require_qr_for_self_check_in
- check_in_start_time
- check_in_end_time
- allow_late_check_in
- allow_attendance_edit_after_submit

## Suggested Entities

Candidate entities:

- religious_attendance_sessions
- religious_attendance_records
- religious_attendance_session_targets
- religious_attendance_edit_history
- religious_attendance_settings

## Session Targets

An attendance session may target:

- service
- group
- congregation
- program group
- event

Use a flexible target model so the same pattern works across religious activities.

## Required Platform Engines

This module must use:

- Organization Engine for tenant context
- Permission Engine for authorization
- Audit Engine for edits after submission
- Reporting Engine for attendance reports
- Notification Engine for reminders
- Timeline Engine for member activity history

## Permission Examples

- religious.attendance.view
- religious.attendance.mark
- religious.attendance.edit
- religious.attendance.close_session
- religious.attendance.reports.view

## Events Published

- ReligiousAttendanceSessionCreated
- ReligiousAttendanceMarked
- ReligiousAttendanceEdited
- ReligiousAttendanceSessionClosed

## Reports

Reports may include:

- Attendance by service
- Attendance by congregation
- Attendance by group
- Attendance by branch
- Attendance trend
- Absent members
- First-time attendees

## Security Requirements

- Attendance records are organization-owned.
- Leaders may only mark attendance within assigned scope.
- Edits after submission should be audited.
- Reports must respect permission and scope.

## Testing Requirements

Tests must cover:

- Create attendance session
- Mark attendance
- Prevent duplicate attendance record
- Edit attendance with audit record
- Permission checks
- Organization boundary checks
- Report scope checks

## Final Rule

Religious attendance must be designed as a reusable capability pattern, not a one-off church-only feature.
