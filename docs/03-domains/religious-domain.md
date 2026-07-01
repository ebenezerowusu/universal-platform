# Religious Domain Specification

## Purpose

The Religious Domain is the first major product domain of Universal Platform.

It provides operational management for religious organizations while still respecting the wider platform vision: the Platform Core must remain domain-agnostic, and religious-specific logic must stay inside this domain.

## Domain Position

```text
Platform Core
  -> Capability Engines
    -> Religious Domain
      -> Religious Modules
```

The Religious Domain uses platform capabilities such as identity, organizations, permissions, SMS, payments, notifications, storage, audit, workflow, localization, and reporting.

The Platform Core must not contain religious-specific concepts.

## Supported Organization Types

The domain should support different religious and faith-based structures, including:

- Churches
- Mosques
- Ministries
- Fellowships
- Prayer groups
- Religious foundations
- Religious networks
- Multi-branch religious institutions

Terminology must be configurable so the platform does not force church-only language.

Examples:

- Member -> Congregant, Worshipper, Believer
- Leader -> Pastor, Imam, Elder, Coordinator
- Branch -> Parish, Assembly, Mosque, District, Campus

## Domain Scope

The Religious Domain includes:

- Organization and branch hierarchy
- Member management
- Visitor management
- Congregations
- Services
- Groups, departments, cells, and study groups
- Bible study / religious education program trees
- Attendance
- Family and household relationships
- Giving and religious payments
- Prayer and counseling
- Welfare and care
- Religious events
- Communication and announcements
- Leadership management
- Follow-ups and tasks
- Religious dashboards and reports

## Out of Scope

The Religious Domain must not implement generic platform services directly.

The following belong to Platform Core or Capability Engines:

- Authentication
- Generic organizations/tenants
- Generic RBAC engine
- SMS provider integration
- Payment provider integration
- Storage provider integration
- Subscription billing engine
- Audit engine
- Notification engine
- Localization engine
- Generic workflow engine

## Main Modules

### 1. Religious Organization Structure

Supports religious branch/network hierarchy.

Features:

- Create religious organization profile
- Create branches
- Connect existing organizations as branches
- Approve/reject branch connection requests
- Disconnect/deconnect branch relationships
- Parent/child hierarchy tree
- Branch reassignment
- Branch suspension/reactivation
- Hierarchical reporting
- Parent organization communication to lower branches

Important relationship statuses:

- pending
- active
- disconnect_requested
- disconnected
- rejected
- suspended
- revoked

Governance settings:

- allow_child_to_request_disconnect
- require_parent_approval_for_disconnect
- allow_parent_to_remove_child_branch
- retain_historical_access_after_disconnect
- disconnect_notice_period_days
- allow_parent_to_view_members
- allow_parent_to_manage_members
- allow_parent_to_assign_leaders
- require_child_approval_for_leader_assignment

### 2. Member Management

Manages religious members/worshippers/congregants.

Features:

- Member registration
- Self-registration
- Admin-created member profiles
- Member photo
- QR member ID
- Branch assignment
- Congregation assignment
- Group assignment
- Status tracking
- Leadership history
- Baptism/confirmation or equivalent religious milestones
- Volunteer/worker status
- Documents
- Notes
- Transfers between branches
- Custom fields

Possible statuses:

- active
- inactive
- transferred
- deceased
- suspended
- under_review

Member lifecycle:

```text
Visitor -> New Convert -> Member -> Worker -> Leader
```

Terminology must be configurable.

### 3. Visitor Management

Tracks first-time visitors and follow-up.

Features:

- Visitor registration
- Visitor check-in
- Invited-by member
- Follow-up assignment
- Notes
- Reminders
- Conversion to member
- Visitor analytics

Visitor statuses:

- new
- contacted
- interested
- converted
- inactive

### 4. Congregations and Services

A congregation represents a group of people.

A service represents a scheduled gathering.

Structure:

```text
Organization
  -> Branch
    -> Congregation
      -> Services
      -> Groups
```

Congregation examples:

- Adult
- Youth
- Teens
- Children
- Language-based congregation
- Gender-based congregation where appropriate

Service examples:

- First Service
- Second Service
- Midweek Service
- Prayer Service
- Children Service
- Special Service

Features:

- Create/edit congregations
- Assign congregation leaders
- Create services under congregations
- Schedule services
- Set venue
- Set attendance rules
- Service reports

### 5. Groups, Departments, Cells, and Study Groups

Groups live inside congregations.

Use one flexible group system with `type`.

Group types:

- department
- cell
- study_group
- fellowship
- ministry
- class
- team

Department examples:

- Choir
- Ushers
- Media
- Protocol
- Technical
- Finance
- Prayer Team

Cell examples:

- Home cell
- Area cell
- Zone fellowship

Study group examples:

- Bible Study Group
- Discipleship Class
- New Converts Class
- Leadership Class

Features:

- Create groups under congregations
- Assign group leaders
- Add/remove members
- Group meeting schedule
- Group attendance
- Announcements
- Group reports
- Optional group hierarchy

### 6. Religious Education / Bible Study Program Tree

The first implementation may focus on Bible Study, but the model should allow other religious education programs.

Structure:

```text
Program
  -> Program Group Tree
    -> Attendance Sessions
      -> Attendance Records
```

Features:

- Create religious education program
- Create unlimited group/subgroup tree
- Assign general leader
- Assign group/subgroup leaders
- Leader access by subtree
- Session creation
- Attendance marking
- Completion reports
- Roll-up attendance reports

The model must avoid forcing Christian-only terminology in the core design. Labels can be configured per organization.

### 7. Attendance

Attendance should use the reusable Attendance Engine.

Attendance applies to:

- Services
- Events
- Groups
- Cells
- Departments
- Religious education programs
- Children classes

Supported attendance methods:

- Manual marking
- Quick buttons / bulk marking
- QR check-in
- GPS/location check-in
- Mobile self check-in
- Staff-assisted check-in
- Offline marking and later sync

Attendance statuses:

- present
- absent
- late
- excused
- pending_review

Attendance settings:

- allow_manual_marking
- allow_quick_buttons
- allow_qr_check_in
- allow_gps_check_in
- allow_mobile_self_check_in
- require_gps_for_self_check_in
- require_qr_for_self_check_in
- allowed_radius_meters
- check_in_start_time
- check_in_end_time
- allow_late_check_in
- allow_attendance_edit_after_submit
- require_leader_approval_for_self_check_in

### 8. Family and Household System

The system should support member-driven family linking and admin review.

Relationship types:

- spouse
- father
- mother
- child
- guardian
- sibling
- relative
- emergency_contact

Relationship statuses:

- pending
- accepted
- rejected
- removed

Linking methods:

- Search by phone/email/member ID
- QR linking
- Invite link through WhatsApp/SMS
- Admin linking

Admin features:

- Review family links
- Approve/reject relationships
- Merge households
- Split households
- Resolve conflicts
- Lock household records

The backend must prevent duplicate and logically impossible relationships where practical.

### 9. Giving and Religious Payments

Religious giving must use the Payment Engine.

Payment types:

- Tithe
- Offering
- Donation
- Welfare support
- Project support
- Event payment
- Pledge payment

Features:

- Member payment from mobile app
- Admin-recorded giving where appropriate
- Payment confirmation
- Digital receipts
- Giving history
- Failed payment handling
- Branch-level giving reports
- Pledge tracking

The Religious Domain must not call Hubtel, Paystack, Flutterwave, or any provider directly.

Correct flow:

```text
Religious Giving
  -> Payment Engine
    -> Payment Provider Resolver
      -> Provider Adapter
```

### 10. Prayer and Counseling

Features:

- Submit prayer request
- Public/private/confidential request options
- Assign to prayer team/leaders
- Track status
- Testimony submission
- Follow-up reminders
- Counseling request
- Counselor assignment
- Appointment scheduling
- Confidential notes

Confidential records require strict permission and audit controls.

### 11. Welfare and Care

Features:

- Welfare requests
- Support cases
- Financial assistance tracking
- Hospital visitation
- Bereavement tracking
- Care follow-up
- Household welfare support
- Welfare reports

### 12. Events

Features:

- Religious event creation
- Free/paid events
- RSVP/registration
- Attendance tracking
- Event reminders
- Volunteers
- Reports
- Recurring events

Paid event flows must use the Payment Engine.

### 13. Communication

Communication must use the Notification Engine and SMS Engine.

Features:

- Announcements
- Bulk messages
- Targeted messages
- Scheduled messages
- Birthday messages
- Anniversary messages
- Event reminders
- Attendance reminders
- Follow-up reminders

Targets:

- Entire organization
- Branch
- Hierarchy subtree
- Congregation
- Group
- Service attendees
- Visitors
- Families/households
- Leaders
- Custom filters

The domain must not call SMS providers directly.

### 14. Leadership Management

Features:

- Assign leaders to branches
- Assign leaders to congregations
- Assign leaders to services
- Assign leaders to groups
- Assign leaders to education program groups
- Acting/temporary leaders
- Leadership transfer
- Leadership history
- Approval workflows
- Audit logs

### 15. Reports and Dashboards

Reports may include:

- Total members
- Active/inactive members
- New members
- Visitors
- Visitor conversion
- Attendance trends
- Giving summary
- Branch comparison
- Group activity
- SMS usage
- Payment summary
- Welfare cases
- Follow-up completion
- Leadership coverage

Reports must respect tenant boundaries and permissions.

## Core Entities

Initial entity candidates:

- religious_profiles
- religious_branch_relations
- religious_members
- religious_member_status_history
- religious_visitors
- religious_congregations
- religious_services
- religious_groups
- religious_group_members
- religious_programs
- religious_program_groups
- religious_family_relationships
- religious_households
- religious_giving_categories
- religious_pledges
- religious_prayer_requests
- religious_counseling_cases
- religious_welfare_cases
- religious_leadership_assignments

Names may change during database design, but domain ownership must remain clear.

## Permission Examples

- religious.members.view
- religious.members.create
- religious.members.update
- religious.members.delete
- religious.members.transfer
- religious.visitors.manage
- religious.attendance.view
- religious.attendance.mark
- religious.attendance.edit
- religious.giving.view
- religious.giving.manage
- religious.prayer.view
- religious.prayer.manage
- religious.counseling.manage
- religious.welfare.manage
- religious.branches.manage
- religious.groups.manage
- religious.reports.view

Permission scopes may include:

- own
- own_group
- branch
- subtree
- organization

## Events Published

Possible events:

- ReligiousMemberCreated
- ReligiousMemberUpdated
- ReligiousMemberTransferred
- ReligiousVisitorRegistered
- ReligiousVisitorConverted
- ReligiousAttendanceMarked
- ReligiousGroupCreated
- ReligiousBranchConnected
- ReligiousBranchDisconnectRequested
- ReligiousGivingRecorded
- ReligiousPrayerRequestSubmitted
- ReligiousCounselingCaseCreated
- ReligiousWelfareCaseCreated

## Events Consumed

Possible consumed events:

- PaymentCompleted
- PaymentFailed
- SmsDelivered
- SmsFailed
- NotificationDelivered
- ModuleInstalled
- SubscriptionChanged

## Configuration

Organization-level configuration may include:

- member_label
- leader_label
- branch_label
- congregation_label
- group_label
- visitor_follow_up_days
- allow_member_self_registration
- require_member_approval
- family_linking_enabled
- attendance_methods_enabled
- giving_categories
- communication_preferences

## Platform Engine Usage

The Religious Domain must use:

- Identity Engine for user identity
- Organization Engine for tenant/branch context
- Permission Engine for authorization
- SMS Engine for text messages
- Payment Engine for giving/event payments
- Notification Engine for in-app/push/email-style notifications
- Storage Engine for documents/photos
- Workflow Engine for approvals/follow-ups
- Audit Engine for sensitive actions
- Reporting Engine for dashboards/reports
- Localization Engine for terminology/language

## Implementation Priority

Recommended Religious Domain implementation order after Platform Core exists:

1. Religious profile and settings
2. Branch hierarchy
3. Member management
4. Congregations and services
5. Groups
6. Attendance integration
7. Visitor management
8. Communication integration
9. Giving/payment integration
10. Family/households
11. Prayer/counseling/welfare
12. Reports

## Final Rule

The Religious Domain is the first product domain, not the Platform Core.

No religious-specific business logic should leak into the Platform Core.
