# Document Media Library UI Contract

## Purpose

This document defines the MVP UI contract for Document and Media Library foundation.

The screens should help organization users browse folders, view document metadata, prepare uploads, manage access, and review domain attachments.

## Navigation Direction

Document/media screens should appear when:

- user is authenticated
- organization is selected
- document/media feature is available
- user has required permission
- feature availability allows the section

## Library Dashboard

Should show:

- recent documents
- folders
- media items
- restricted items count where available
- upload action where permitted

## Folder List

Should show:

- folder name
- parent folder where available
- visibility
- status
- item count where available
- edit action where permitted

## Document List

Should show:

- title
- file type
- category
- access level
- status
- created date
- action to view detail

## Document Detail

Should show:

- title
- description where available
- file type
- category
- access level
- storage reference status
- linked domain attachments
- audit-friendly timestamps where available

## Upload Placeholder

Should support:

- title
- category
- file type
- folder selection
- access level
- upload intent action

The UI should not call storage providers directly.

## Domain Attachments

Should show:

- domain type
- linked record safe label where available
- attachment label
- created date
- remove action where permitted

## Access Warning States

Should show clear warnings for:

- restricted document
- missing storage reference
- archived document
- unavailable file
- access denied

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 30:

- advanced media streaming UI
- OCR/document AI UI
- e-signature UI
- unrestricted public sharing UI
- provider-specific storage UI

## Final Rule

Document UI should manage metadata and access while leaving storage-provider behavior behind platform APIs.
