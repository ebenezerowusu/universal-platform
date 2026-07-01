# Religious Family and Household Specification

## Purpose

This document defines family relationships and households inside the Religious Domain.

Family and household features help organizations understand member relationships, emergency contacts, household care needs, family attendance, and welfare support.

## Scope

The family and household module may include:

- Family relationship requests
- Relationship approval
- Household suggestions
- Household creation
- Household merge/split
- Emergency contact links
- Family attendance views
- Household communication
- Welfare/care connection later

## Relationship Types

Possible relationship types:

- spouse
- father
- mother
- child
- guardian
- sibling
- relative
- emergency_contact

Labels may be configurable later.

## Relationship Statuses

Possible statuses:

- pending
- accepted
- rejected
- removed

## Linking Methods

Family links may be created through:

- Admin linking
- Member search by phone/email/member code
- QR invitation
- Invite link through notification/SMS later

All methods should use the same backend relationship rules.

## Household Flow

```text
Relationship accepted
  -> Household suggestion generated
  -> Admin reviews where needed
  -> Household created or updated
  -> Household timeline updated
```

## Suggested Entities

Candidate entities:

- religious_family_relationships
- religious_family_relationship_requests
- religious_households
- religious_household_members
- religious_household_history

## Duplicate and Conflict Rules

The system should prevent or flag obvious relationship conflicts.

Examples:

- duplicate relationship request
- circular parent/child relationship
- conflicting household claims

Complex conflicts should be flagged for admin review rather than silently accepted.

## Required Platform Engines

This module must use:

- Organization Engine for tenant context
- Permission Engine for access control
- Workflow Engine for review process where needed
- Notification Engine for relationship requests
- Audit Engine for household changes
- Timeline Engine for relationship history

## Permission Examples

- religious.family.view
- religious.family.manage
- religious.family.approve_relationship
- religious.households.view
- religious.households.manage

## Events Published

- ReligiousFamilyRelationshipRequested
- ReligiousFamilyRelationshipAccepted
- ReligiousFamilyRelationshipRejected
- ReligiousHouseholdCreated
- ReligiousHouseholdUpdated

## Security Requirements

- Family data is sensitive and organization-owned.
- Relationship changes should be auditable.
- Household access must be permission-controlled.
- Members should not be able to create unchecked sensitive relationships without appropriate validation or review.

## Testing Requirements

Tests must cover:

- Request relationship
- Accept relationship
- Reject relationship
- Suggest household
- Merge household
- Permission checks
- Organization boundary checks
- Conflict detection

## Final Rule

Family and household records belong to the Religious Domain and must be treated as sensitive relationship data.
