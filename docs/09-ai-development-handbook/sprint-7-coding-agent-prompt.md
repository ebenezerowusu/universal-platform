# Sprint 7 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 7 after the Flutter foundation and Religious backend foundation are stable.

Sprint 7 should implement the first Religious MVP Flutter workflows.

## Prompt

```text
You are implementing Sprint 7 for Universal Platform.

Your task is to implement the first Religious MVP Flutter workflows using the existing Flutter foundation and backend API contracts.

Required reading before coding:
1. CLAUDE.md
2. docs/11-ui-ux/11-flutter-ui-ux-standards.md
3. docs/11-ui-ux/flutter-app-shell-contract.md
4. docs/11-ui-ux/flutter-api-client-contract.md
5. docs/11-ui-ux/flutter-state-navigation-implementation-flow.md
6. docs/11-ui-ux/religious-mvp-flutter-screens-contract.md
7. docs/12-api/religious-settings-api-contract.md
8. docs/12-api/religious-member-visitor-api-contract.md
9. docs/12-api/religious-structure-attendance-api-contract.md
10. docs/12-api/frontend-backend-integration-checklist.md
11. docs/16-quality/sprint-7-quality-gates.md

Implement only:
- Religious navigation entry points
- Religious dashboard placeholder or summary screen
- member list screen
- member detail placeholder or foundation
- create member form foundation
- visitor list screen
- create visitor form foundation
- visitor conversion action foundation where backend is ready
- congregation/service/group list screens or placeholders
- attendance session list screen
- attendance marking foundation where backend is ready
- API client methods for the above contracts
- loading, empty, error, and unavailable states
- tests for key screens and API parsing where practical

Do not implement:
- full offline sync
- QR attendance
- GPS attendance
- live SMS sending UI
- live payment/giving UI
- full family/household UI
- full care/counseling/welfare UI
- advanced reports
- POS or Commerce UI

Recommended branch:
feature/religious-flutter-mvp

Recommended PR title:
feat: add Religious MVP Flutter workflows

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- screens/components added
- API methods added
- tests added
- checks run
- architecture rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused PR that turns the Religious MVP backend foundation into usable Flutter workflows without adding advanced features.

## Final Rule

Sprint 7 connects the user interface to the MVP domain. It should use existing app shell, API client, state, navigation, and design system foundations instead of creating disconnected screens.
