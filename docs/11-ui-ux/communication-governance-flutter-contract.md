# Communication Governance Flutter Contract

## Purpose

This document defines the MVP Flutter screen contract for Communication Governance.

The screens should help organization users manage preferences, templates, recipient segments, and send review without exposing provider-specific details.

## Navigation Direction

Communication Governance screens should appear under the Communication area when:

- user is authenticated
- organization is selected
- Communication module is available
- user has required permission
- feature availability allows the section

## Preferences Screen

Purpose:

- view recipient communication preferences
- update SMS allowed status where permitted
- review opt-out status

Suggested fields:

- recipient name or safe label
- recipient type
- SMS allowed
- last updated date
- updated by where available

## Templates Screen

Purpose:

- list reusable message templates
- create or edit template where permitted

Template fields:

- name
- channel
- body preview
- status
- updated date

## Template Editor

Should support:

- template name
- SMS body
- status
- save action
- validation feedback

MVP can keep personalization simple and avoid advanced variables beyond documented placeholders.

## Segments Screen

Purpose:

- list saved recipient segments
- create or edit segment where permitted

Segment fields:

- name
- source
- rule summary
- estimated recipient count where available

## Segment Editor

Should support simple source selection:

- manual list
- active Religious members where permitted
- visitors where permitted
- selected group where permitted

## Recipient Preview Screen

Should show:

- total recipients
- eligible recipients
- excluded recipients
- estimated SMS units
- warning messages where applicable

Do not expose private recipient details unnecessarily.

## Send Review Screen

Purpose:

- show final send-readiness result before sending

Should show:

- can send or cannot send
- eligible count
- excluded count
- estimated units
- wallet warning where applicable
- preference warning where applicable

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 18:

- campaign automation builder
- WhatsApp screens
- email screens
- AI generated messages
- advanced analytics
- provider configuration screens

## Final Rule

Flutter should guide users toward safe communication decisions without exposing provider complexity or recipient data unnecessarily.
