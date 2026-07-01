# HR Domain Specification

## Purpose

The HR Domain provides staff and employee management capabilities for organizations.

It is separate from member/customer management. In the Religious Domain, members are religious/community participants. In the HR Domain, staff are employees or workers of the organization.

## Scope

The HR Domain may include:

- Staff profiles
- Employment records
- Departments
- Job roles
- Staff attendance
- Leave management
- Staff documents
- Payroll foundation later
- Performance tracking later

## Out of Scope

The HR Domain must not own platform identity.

A staff profile may link to a platform user, but the Identity Engine owns authentication.

Payment processing belongs to the Payment Engine.

Financial accounting belongs to the Finance Domain.

## Main Modules

### Staff Profiles

Features:

- Staff record
- Contact details
- Employment status
- Department
- Job role
- Start date
- Documents

### Departments and Roles

Features:

- HR departments
- Job roles
- Reporting lines later
- Staff assignment

### Staff Attendance

May reuse Attendance Engine patterns.

Features:

- Clock-in/clock-out later
- Daily attendance
- Attendance reports
- Exception notes

### Leave Management

Features:

- Leave types
- Leave request
- Approval workflow
- Leave balance
- Leave calendar

### Payroll Foundation Later

Future features:

- Salary records
- Allowances
- Deductions
- Payslips
- Payroll approvals

Payroll must be designed carefully and may require separate ADRs.

## Suggested Entities

Candidate entities:

- hr_staff_profiles
- hr_departments
- hr_job_roles
- hr_employment_records
- hr_leave_types
- hr_leave_requests
- hr_leave_balances
- hr_staff_documents
- hr_attendance_records

## Permission Examples

- hr.staff.view
- hr.staff.create
- hr.staff.update
- hr.departments.manage
- hr.leave.view
- hr.leave.request
- hr.leave.approve
- hr.attendance.view
- hr.reports.view

## Events Published

- HrStaffCreated
- HrStaffUpdated
- HrLeaveRequested
- HrLeaveApproved
- HrLeaveCancelled
- HrAttendanceRecorded

## Events Consumed

- UserCreated
- FileUploaded
- WorkflowTaskCompleted
- NotificationDelivered

## Platform Engine Usage

HR Domain must use:

- Identity Engine for linked user accounts
- Organization Engine for tenant context
- Permission Engine for HR permissions
- Storage Engine for staff documents
- Workflow Engine for leave approvals
- Notification Engine for reminders
- Reporting Engine for HR reports
- Audit Engine for sensitive staff changes

## Implementation Priority

HR should be introduced after Platform Core foundations are stable.

Recommended order:

1. Staff profiles
2. Departments and roles
3. Staff documents
4. Leave types
5. Leave requests
6. Workflow integration
7. Attendance
8. Reports
9. Payroll foundation later

## Final Rule

HR owns employee/staff operations. It must not be confused with Religious members or platform users.
