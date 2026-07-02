# Feedback Feature Requests Roadmap Intake UI Contract

## Purpose

This document defines the MVP UI contract for Feedback, Feature Requests, and Product Roadmap Intake foundation.

The screens should help users submit feedback, help support/product teams triage requests, group duplicates, review prioritization signals, manage roadmap candidates, and communicate status updates without promising delivery.

## Navigation Direction

Feedback intake screens should appear when:

- user is authenticated
- organization is selected where required
- feedback intake feature is available
- user has required permission
- visibility rules allow access

## Feedback Dashboard

Should show:

- open feedback item count
- open feature request count
- duplicate group count
- roadmap candidate count
- items awaiting response count
- recent feedback audit events

## Feedback Submission Placeholder

Should allow a user to submit:

- source channel placeholder
- feedback type
- short summary
- optional safe details
- related module placeholder
- attachment placeholder through Document/Media later

## Feedback Item List

Should show:

- feedback safe summary
- source channel
- feedback type
- submitted by safe label
- related module placeholder
- status
- action to view detail

## Feature Request List

Should show:

- request key/title
- owner module/domain
- problem statement safe summary
- desired outcome safe summary
- affected role placeholder
- status

## Category List

Should show:

- category name
- description safe summary
- owner module/domain
- active item count placeholder
- status

## Duplicate and Group Review

Should show:

- group safe label
- representative request safe summary
- grouped item count
- affected organizations count placeholder
- confidence placeholder
- add/remove placeholder actions where permitted

## Prioritization Summary

Should show:

- request safe label
- impact placeholder
- frequency placeholder
- affected organizations count placeholder
- effort placeholder
- risk placeholder
- revenue signal placeholder

Prioritization scores are signals, not automatic decisions.

## Roadmap Candidate List

Should show:

- candidate safe label
- linked feature request
- product area/module
- review status
- release/announcement link where available
- non-commitment notice

## Linked Support and Help View

Should show:

- linked support cases
- linked help articles
- knowledge base gap indicator
- linked feedback items
- permission/masking indicators

## Status Response List

Should show:

- response safe label
- target feedback/request
- response status
- sent by safe label
- sent date placeholder
- response channel placeholder

## Feedback Audit History

Should show:

- event type
- actor safe label
- target safe label
- timestamp
- safe metadata summary

## User Experience Placeholder

Users should be able to:

- submit feedback
- see status of their submitted feedback where permitted
- view safe responses
- understand that roadmap candidates are not delivery commitments

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied
- no feedback yet

## MVP Exclusions

Do not implement in Sprint 60:

- public voting marketplace UI
- committed roadmap delivery UI
- AI-only priority decision UI
- cross-tenant private feedback browser UI
- replacement support desk UI
- product planning suite UI

## Final Rule

Feedback intake UI should make user signals visible and actionable while avoiding implied promises about delivery or roadmap timing.
