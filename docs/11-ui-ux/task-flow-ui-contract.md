# Task Flow UI Contract

## Purpose

This document defines the MVP UI contract for workflow task and approval screens.

The screens should help users review tasks, make decisions, view status history, and manage basic workflow placeholders where permitted.

## Navigation Direction

Task-flow screens should appear when:

- user is authenticated
- organization is selected
- workflow feature is available
- user has required permission
- feature availability allows workflows

## Dashboard

Should show:

- pending tasks
- tasks assigned to me
- approved/rejected counts for selected period
- manual review count where available
- reminder placeholder count

## Task Inbox

Should show:

- task title
- workflow title
- domain type
- safe summary
- assigned date
- status
- action to view detail

## Task Detail

Should show:

- workflow title
- instance status
- domain reference safe label
- current step
- assigned reviewer where safe
- decision history
- source record link where permitted

## Decision Screen

Should support:

- approve action
- reject action
- return placeholder action
- decision note
- confirmation prompt

## Status History

Should show:

- old status
- new status
- actor safe label
- timestamp
- note where permitted

## Definition Admin Placeholder

Admin users may see:

- workflow key
- title
- module/domain
- status
- step count
- edit placeholder where permitted

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- decision progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 34:

- full visual builder UI
- BPMN visual designer
- arbitrary script configuration
- AI workflow generation UI
- unrestricted auto-approval UI

## Final Rule

Task-flow UI should make approvals clear, safe, and auditable without overbuilding automation design tools in MVP.
