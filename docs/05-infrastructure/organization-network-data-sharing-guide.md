# Organization Network Data Sharing Guide

## Purpose

This document defines operating guidance for organization relationships, branch connection requests, disconnect/remove requests, relationship history, hierarchy metadata, data sharing policies, historical access policies, network reporting visibility, and network audit history.

The goal is to support parent/child organization networks without weakening tenant ownership.

## Principle

A relationship between organizations is not a transfer of ownership.

Parent visibility must be explicit, scoped, permissioned, privacy-aware, and auditable.

## Data Sources

Organization network operations may use:

- organization relationship metadata
- connection request
- approval/rejection placeholder
- disconnect/remove request placeholder
- relationship status history
- hierarchy path/depth metadata placeholder
- data sharing policy placeholder
- historical access policy placeholder
- network reporting visibility placeholder
- audit records

## Relationship Direction

Organization relationship records should capture:

```text
root_organization_id placeholder
parent_organization_id
child_organization_id
relationship_type
level_depth_placeholder
status
created_by
```

## Connection Request Direction

Connection requests should capture:

- requesting organization
- target organization
- request type
- requested by
- review status
- approved/rejected placeholder
- safe notes

## Disconnect and Remove Direction

Disconnect/remove placeholders should capture:

- relationship
- requested by
- request type
- notice period placeholder
- review status
- final status placeholder

Disconnecting should not delete child organization data.

## Relationship History Direction

Relationship history should capture:

- relationship
- old status
- new status
- actor safe label
- reason safe summary
- timestamp

## Hierarchy Direction

Hierarchy metadata may include:

- path placeholder
- depth placeholder
- level label placeholder
- root organization placeholder

Hierarchy changes should be recalculated through controlled processes where practical.

## Data Sharing Direction

Data sharing policies may define:

- no access
- summary reporting only
- selected module reporting placeholder
- selected entity visibility placeholder
- historical access placeholder
- inherited setting placeholder

## Historical Access Direction

Historical access policies may define:

- no historical access
- summary-only access placeholder
- audit-only access placeholder
- retained access placeholder
- manual review required

Disconnected branches should not expose future data to former parent organizations unless explicitly allowed by policy.

## Reporting Direction

Cross-organization reporting should respect:

- data sharing policy
- user permission
- privacy masking
- child organization ownership
- historical access rules

## Audit Direction

Audit should record:

- relationship created or updated
- connection requested
- connection approved/rejected placeholder
- disconnect requested
- disconnect approved/rejected placeholder
- data sharing policy changed
- historical access policy changed
- network reporting visibility changed

## Data Quality Warnings

Warn or flag when:

- relationship creates a cycle
- parent/child organization is inactive
- data sharing policy is missing
- historical access policy is missing for disconnected branch
- reporting visibility exceeds sharing policy
- relationship status conflicts with hierarchy path
- branch disconnect has unresolved handover/export tasks

## Final Rule

Organization network operations must keep hierarchy, visibility, ownership, privacy, and audit history aligned.
