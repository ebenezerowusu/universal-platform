# Sprint 30 Document Media Library Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 30.

Sprint 30 prepares the Document and Media Library foundation after calendar, HR, finance, commerce, religious, and operations modules have introduced many file/reference needs.

## Prompt

```text
You are implementing Sprint 30 for Universal Platform.

Your task is to implement the Document and Media Library foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/04-capability-engines/storage-engine.md
3. docs/04-capability-engines/permission-engine.md
4. docs/04-capability-engines/audit-engine.md
5. docs/14-roadmap/sprint-30-document-media-library-roadmap.md
6. docs/13-module-sdk/document-media-library-readiness.md
7. docs/12-api/document-media-library-api-contract.md
8. docs/11-ui-ux/document-media-library-ui-contract.md
9. docs/05-infrastructure/document-media-storage-operations-guide.md
10. docs/16-quality/sprint-30-document-media-library-review.md

Implement only:
- document/media feature registration foundation
- library/folder foundation where practical
- document metadata foundation
- storage reference foundation
- domain attachment/link foundation
- access level and visibility foundation
- file type/category foundation
- upload intent placeholder where practical
- download/view permission checks
- audit records for important document actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- live virus scanning engine
- full OCR or document AI
- public CDN optimization
- advanced video streaming
- e-signature workflows
- unlimited public file sharing
- provider-specific storage calls inside domain modules
- unrelated Commerce/POS expansion

Recommended branch:
feature/document-media-library-foundation

Recommended PR title:
feat: add document media library foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Storage/Permission/Audit boundaries followed
- document access rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused document/media foundation that stores metadata, links files to domain records, enforces access, and delegates storage-provider details to Storage Engine.

## Final Rule

Documents and media must be organization-scoped, access-controlled, auditable, and provider-agnostic. Domain modules should link to document records instead of owning file storage logic.
