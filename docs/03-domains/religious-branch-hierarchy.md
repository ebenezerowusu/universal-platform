# Religious Branch Hierarchy Specification

## Purpose

This document defines religious organization branch hierarchy inside the Religious Domain.

The Platform Core provides organizations and tenant foundation. Religious branch hierarchy rules belong to the Religious Domain.

## Scope

The branch hierarchy module may include:

- Parent and child organization relationships
- Branch creation
- Branch connection requests
- Branch approval flow
- Branch disconnection flow
- Hierarchy tree view
- Branch reassignment
- Branch reporting scope
- Parent access rules
- Governance settings

## Relationship Model

Religious organizations may be connected in a hierarchy.

Example:

```text
Global Organization
  -> National Office
    -> Region
      -> District
        -> Branch
```

Hierarchy labels should be configurable.

## Relationship Statuses

Possible statuses:

- pending
- active
- disconnect_requested
- disconnected
- rejected
- suspended
- revoked

## Governance Settings

Possible settings:

- allow_child_to_request_disconnect
- require_parent_approval_for_disconnect
- allow_parent_to_remove_child_branch
- retain_historical_access_after_disconnect
- allow_parent_to_view_member_summary
- allow_parent_to_assign_leaders
- require_child_approval_for_leader_assignment

## Suggested Entities

Candidate entities:

- religious_branch_relations
- religious_branch_relation_history
- religious_branch_governance_settings
- religious_branch_access_policies

## Access Levels

Parent access to child data should be configurable.

Possible levels:

- none
- summary_only
- view_basic_profile
- view_full_profile
- manage_records

Sensitive records should still require explicit permissions.

## Events Published

- ReligiousBranchConnectionRequested
- ReligiousBranchConnectionApproved
- ReligiousBranchConnectionRejected
- ReligiousBranchDisconnectRequested
- ReligiousBranchDisconnected
- ReligiousBranchSuspended
- ReligiousBranchReassigned

## Required Platform Engines

This module must use:

- Organization Engine for organization records
- Permission Engine for authorization
- Workflow Engine for approvals
- Audit Engine for sensitive changes
- Reporting Engine for hierarchy reports
- Notification Engine for branch requests

## Security Requirements

- Parent access must be explicit.
- Branch relationship changes must be audited.
- Sensitive child data must not be exposed by default.
- Organization boundaries must still be enforced.

## Testing Requirements

Tests must cover:

- Request branch connection
- Approve connection
- Reject connection
- Request disconnection
- Parent access level behavior
- Hierarchy reporting scope
- Permission checks
- Audit records

## Final Rule

Religious hierarchy is a domain rule. Platform Core must only provide generic organization foundation.
