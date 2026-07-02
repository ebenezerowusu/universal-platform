# Sprint 37 Backup Restore Disaster Recovery Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 37.

Sprint 37 prepares the Backup, Restore, and Disaster Recovery foundation after audit, retention, import/export, documents, reporting, workflow, and operations modules have introduced critical data protection needs.

## Prompt

```text
You are implementing Sprint 37 for Universal Platform.

Your task is to implement the Backup, Restore, and Disaster Recovery foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/05-infrastructure/05-deployment-strategy.md
3. docs/05-infrastructure/backup-restore-drill-playbook.md
4. docs/04-capability-engines/audit-engine.md
5. docs/04-capability-engines/permission-engine.md
6. docs/14-roadmap/sprint-37-backup-restore-dr-roadmap.md
7. docs/13-module-sdk/backup-restore-dr-readiness.md
8. docs/12-api/backup-restore-dr-api-contract.md
9. docs/11-ui-ux/backup-restore-dr-ui-contract.md
10. docs/05-infrastructure/backup-restore-dr-operations-guide.md
11. docs/16-quality/sprint-37-backup-restore-dr-review.md

Implement only:
- backup/restore feature registration foundation
- backup policy metadata foundation
- backup run metadata foundation
- backup verification/checksum placeholder where practical
- restore request placeholder foundation
- restore drill record foundation
- recovery point objective and recovery time objective metadata placeholders
- tenant/organization restore boundary checks
- incident recovery note placeholder where practical
- permission and feature checks
- audit records for backup/restore actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- live production restore automation
- destructive restore without approval
- cross-organization restore
- external backup provider integration
- database engine-specific scripts inside domain modules
- unmanaged file-system access from domain modules
- full business continuity suite
- unrelated Commerce/POS expansion

Recommended branch:
feature/backup-restore-dr-foundation

Recommended PR title:
feat: add backup restore disaster recovery foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Backup/Storage/Audit boundaries followed
- restore safety rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused backup/restore foundation for policies, backup runs, verification placeholders, restore requests, restore drills, recovery objectives, incident notes, and audit-safe recovery operations.

## Final Rule

Backup and restore must be organization-aware, permission-protected, auditable, tested through drills, and non-destructive by default. Restore operations should never bypass tenant boundaries or audit history.
