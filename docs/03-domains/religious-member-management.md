# Religious Member Management Specification

## Purpose

This document defines the Member Management module inside the Religious Domain.

It must remain domain-specific and must not leak member concepts into the Platform Core.

## Scope

Member Management may include:

- Member registration
- Member self-registration
- Admin-created member profiles
- Member status tracking
- Branch assignment
- Congregation assignment
- Group assignment
- Custom fields
- Profile photo
- Documents
- Notes
- Member transfer
- Member history

## Relationship to Platform User

A religious member is not always a platform user.

Some members may have login accounts. Others may exist only as organization records.

If a member needs app access, the member profile may link to a platform user identity.

Identity remains owned by the Identity Engine.

## Member Lifecycle

Possible lifecycle:

```text
Visitor -> New Convert -> Member -> Worker -> Leader
```

Terminology should be configurable.

## Member Statuses

Possible statuses:

- active
- inactive
- transferred
- deceased
- suspended
- under_review

Status changes should be tracked in history.

## Suggested Entities

Candidate entities:

- religious_members
- religious_member_status_history
- religious_member_assignments
- religious_member_notes
- religious_member_documents
- religious_member_custom_values

## Required Platform Engines

This module must use:

- Organization Engine for tenant context
- Permission Engine for access control
- Storage Engine for photos/documents
- Audit Engine for sensitive changes
- Timeline Engine for member history
- Import/Export Engine for member imports/exports
- Localization Engine for labels

## Permission Examples

- religious.members.view
- religious.members.create
- religious.members.update
- religious.members.transfer
- religious.members.import
- religious.members.export
- religious.members.view_sensitive

## Events Published

- ReligiousMemberCreated
- ReligiousMemberUpdated
- ReligiousMemberStatusChanged
- ReligiousMemberTransferred
- ReligiousMemberDocumentUploaded

## Import Support

Member import should use the Import/Export Engine.

Duplicate detection may consider:

- phone
- email
- member code
- name plus date of birth where available

## Security Requirements

- Member records are organization-owned.
- Sensitive fields require stricter permissions.
- Exports must be permission-controlled.
- Important changes should be audited.

## Testing Requirements

Tests must cover:

- Create member
- Update member
- Status change history
- Organization boundary enforcement
- Permission checks
- Duplicate handling
- Document upload through Storage Engine

## Final Rule

Members belong to the Religious Domain. Platform Core must not know religious member rules.
