# Sprint 6 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 6 after the backend MVP foundations are stable.

Sprint 6 should implement the Flutter client foundation only.

## Prompt

```text
You are implementing Sprint 6 for Universal Platform.

Your task is to implement the Flutter client foundation only.

Required reading before coding:
1. CLAUDE.md
2. docs/01-platform-constitution/01-platform-constitution.md
3. docs/11-ui-ux/11-flutter-ui-ux-standards.md
4. docs/11-ui-ux/flutter-client-build-sequence.md
5. docs/11-ui-ux/flutter-state-management-and-navigation.md
6. docs/11-ui-ux/flutter-mobile-app-spec.md
7. docs/11-ui-ux/flutter-desktop-admin-spec.md
8. docs/11-ui-ux/design-system-component-catalog.md
9. docs/14-roadmap/sprint-6-flutter-foundation-plan.md
10. docs/16-quality/sprint-6-quality-gates.md

Implement only:
- Flutter project foundation
- shared app shell
- startup flow
- API client foundation
- standard response/error parsing foundation
- shared app state foundation
- selected organization state foundation
- mobile shell placeholder
- desktop/admin shell placeholder
- shared design system foundation
- login screen placeholder or foundation
- organization selector placeholder or foundation
- tests for startup, shell, and shared components where practical

Do not implement:
- full Religious feature screens
- payment screens
- SMS screens
- POS or Commerce UI
- full offline sync
- app store packaging
- production push notifications
- advanced animations before core flows work

Recommended branch:
feature/flutter-client-foundation

Recommended PR title:
feat: add Flutter client foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- screens/components added
- state areas added
- tests added
- checks run
- architecture rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused PR that creates the Flutter shell and shared client foundation without rushing into feature-heavy screens.

## Final Rule

Sprint 6 creates the client foundation. Do not build disconnected feature pages before shell, state, routing, API client, and design system foundations are stable.
