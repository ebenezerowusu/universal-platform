# Organization Network Data Sharing UI Contract

## Purpose

This document defines the MVP UI contract for Organization Network, Branch Relationships, and Data Sharing Governance foundation.

The screens should help permitted users review organization networks, parent/child relationships, connection requests, disconnect requests, data sharing policies, historical access policies, reporting visibility, and network audit history.

## Navigation Direction

Organization network screens should appear when:

- user is authenticated
- organization is selected
- organization network feature is available
- user has required permission
- feature availability allows organization network visibility

## Network Dashboard

Should show:

- parent relationship count
- child relationship count
- pending connection request count
- pending disconnect request count
- data sharing warning count
- recent network audit events

## Relationship Tree/List

Should show:

- organization safe label
- relationship type
- level/depth placeholder
- status
- parent/child indicator
- action to view detail

## Relationship Detail

Should show:

- parent organization safe label
- child organization safe label
- relationship type
- level label placeholder
- status
- data sharing policy summary
- historical access policy summary
- status history summary

## Connection Request List

Should show:

- requesting organization safe label
- target organization safe label
- request type
- status
- requested date
- approve/reject placeholder actions where permitted

## Disconnect or Remove Request Placeholder

Should show:

- relationship safe label
- requested by safe label
- request reason safe summary
- notice period placeholder
- status
- approve/reject placeholder actions where permitted

## Data Sharing Policy Screen

Should show:

- relationship safe label
- visibility level
- allowed module/reporting placeholders
- privacy warning count
- inherited setting indicator
- status

## Historical Access Policy Screen

Should show:

- disconnected relationship safe label where applicable
- future visibility rule
- historical summary access placeholder
- audit-only access placeholder
- manual review status

## Network Reporting Visibility

Should show:

- reporting scope safe label
- included child organizations count
- allowed modules placeholder
- privacy/masking indicators
- status

## Network Audit History

Should show:

- event type
- actor safe label
- target organization safe label
- timestamp
- safe metadata summary

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 54:

- unrestricted cross-tenant browser UI
- automatic branch takeover UI
- destructive disconnect UI
- hidden parent access UI
- full legal governance UI
- raw child data browsing UI

## Final Rule

Organization network UI should make relationships and visibility policies explicit without giving parent organizations hidden or unlimited access to child data.
