# Sprint 58 Knowledge Base Help Center Self Service Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 58.

Sprint 58 prepares the Knowledge Base, Help Center, and Self-Service Support foundation after policy library, support operations, document/media, search/discovery, notification, workflow, risk controls, and user access foundations have introduced help content and self-service support needs.

## Prompt

```text
You are implementing Sprint 58 for Universal Platform.

Your task is to implement the Knowledge Base, Help Center, and Self-Service Support foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/12-api/document-media-library-api-contract.md
3. docs/12-api/search-discovery-api-contract.md
4. docs/12-api/platform-support-operations-api-contract.md
5. docs/12-api/policy-library-attestations-training-api-contract.md
6. docs/14-roadmap/sprint-58-knowledge-base-help-center-self-service-roadmap.md
7. docs/13-module-sdk/knowledge-base-help-center-self-service-readiness.md
8. docs/12-api/knowledge-base-help-center-self-service-api-contract.md
9. docs/11-ui-ux/knowledge-base-help-center-self-service-ui-contract.md
10. docs/05-infrastructure/knowledge-base-help-center-self-service-guide.md
11. docs/16-quality/sprint-58-knowledge-base-help-center-self-service-review.md

Implement only:
- knowledge base feature registration foundation
- help article metadata foundation
- article version placeholder foundation
- help category/collection foundation
- visibility/audience rule placeholder where practical
- search indexing linkage placeholder where practical
- support escalation linkage placeholder where practical
- article feedback/usefulness placeholder foundation
- localization placeholder where practical
- related policy/training link placeholder where practical
- permission and feature checks
- audit records for help content and self-service support actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- full CMS replacement
- AI support bot as source of truth
- unrestricted public help publishing
- bypass of document or policy permissions
- cross-organization private article visibility
- arbitrary script widgets in help articles
- replacement of Support Operations
- unrelated Commerce/POS expansion

Recommended branch:
feature/knowledge-base-help-center-self-service-foundation

Recommended PR title:
feat: add knowledge base help center self service foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Knowledge Base/Search/Support/Document boundaries followed
- article visibility and self-service rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused knowledge base foundation for help articles, article versions, categories/collections, visibility rules, search linkage, support escalation links, feedback, localization placeholders, related policy/training links, and audit-safe help content operations.

## Final Rule

Self-service support must make help content discoverable and governed without replacing Support Operations, Document/Media, Search, or Policy Library. Help content visibility must be explicit and permission-aware.
