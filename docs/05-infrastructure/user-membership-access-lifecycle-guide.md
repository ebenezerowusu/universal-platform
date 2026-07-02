# User Membership Access Lifecycle Guide

## Purpose

This document defines operating guidance for invitations, memberships, role assignments, permission group placeholders, access history, offboarding placeholders, and privileged access review linkage.

The goal is to make user access controlled, explainable, and auditable.

## Principle

User access should be granted deliberately, changed explicitly, and removed safely.

No domain module should grant access independently of Identity, Organization, and Permission boundaries.

## Data Sources

User access lifecycle operations may use:

- invitation request
- invitation acceptance/expiry placeholder
- organization membership status
- role assignment
- permission group assignment placeholder
- access change history
- offboarding placeholder
- access review linkage placeholder
- audit records

## Invitation Direction

Invitation records should capture:

```text
organization_id
invited_email optional
invited_phone_placeholder optional
role_keys placeholder
invited_by
status
expires_at optional
accepted_by optional
created_at
```

## Membership Direction

Membership records should capture:

```text
organization_id
user_id
status
joined_at optional
removed_at optional
last_status_change_at
```

## Role Assignment Direction

Role assignments should capture:

```text
organization_id
user_id
role_key
assigned_by
assigned_at
status
reason optional
```

Privileged roles should require stronger permission and audit checks.

## Access History Direction

Access history should capture:

```text
organization_id
user_id
event_type
actor
old_value placeholder
new_value placeholder
reason optional
created_at
```

## Offboarding Direction

Offboarding placeholders should check:

- active roles
- privileged permissions
- pending approvals or assigned tasks placeholder
- ownership responsibilities placeholder
- replacement owner placeholder
- access removal status

## Access Review Direction

Privileged users should be linkable to governance access reviews.

Prioritize:

- organization owners/admins
- finance users
- HR/workforce users
- export users
- support/internal users

## Notification Direction

Notifications may be triggered for:

- invitation created
- invitation accepted placeholder
- role assigned
- privileged role assigned
- membership suspended
- offboarding started
- offboarding completed placeholder

Use Notification Engine where practical.

## Audit Direction

Audit should record:

- invitation created/cancelled/accepted placeholder
- membership activated/suspended/removed
- role assigned/removed
- permission group changed placeholder
- offboarding started/completed placeholder
- access review linked placeholder

## Data Quality Warnings

Warn or flag when:

- invitation is expired but still pending
- user has privileged role without recent access review
- inactive user still has active role
- role assignment references invalid role
- user belongs to wrong organization context
- offboarding user still has active permissions

## Final Rule

User access lifecycle operations must keep identity, membership, role, permission, and audit history aligned.
