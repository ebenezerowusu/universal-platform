# Knowledge Base Help Center Self Service API Contract

## Purpose

This document defines the MVP API contract for Knowledge Base, Help Center, and Self-Service Support foundation.

The API should support help center dashboards, help articles, article versions, categories, collections, visibility rules, search linkage, feedback, support escalation links, related content links, and help audit history while preserving organization boundaries.

## Owner

```text
Knowledge Base Foundation
Permission Engine
Audit Engine
Document/Media Foundation
Search/Discovery Foundation
Support Operations Foundation
Policy Library Foundation
Localization Foundation
```

## Base Path

```text
/api/v1/organizations/{organizationId}/knowledge-base
```

## Help Center Dashboard

```text
GET /api/v1/organizations/{organizationId}/knowledge-base/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "publishedArticles": 120,
    "draftArticles": 8,
    "openFeedbackItems": 16,
    "supportEscalationsFromArticles": 9,
    "outdatedArticles": 4
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Help Articles

```text
GET /api/v1/organizations/{organizationId}/knowledge-base/articles
POST /api/v1/organizations/{organizationId}/knowledge-base/articles
GET /api/v1/organizations/{organizationId}/knowledge-base/articles/{articleId}
PATCH /api/v1/organizations/{organizationId}/knowledge-base/articles/{articleId}
POST /api/v1/organizations/{organizationId}/knowledge-base/articles/{articleId}/retire-placeholder
```

### Create Article Request

```json
{
  "articleKey": "getting-started-placeholder",
  "ownerModuleDomain": "platform",
  "title": "Getting started",
  "categoryId": "...",
  "visibilityRulePlaceholder": {
    "type": "organization_only"
  }
}
```

## Article Versions

```text
GET /api/v1/organizations/{organizationId}/knowledge-base/articles/{articleId}/versions
POST /api/v1/organizations/{organizationId}/knowledge-base/articles/{articleId}/versions
GET /api/v1/organizations/{organizationId}/knowledge-base/article-versions/{versionId}
PATCH /api/v1/organizations/{organizationId}/knowledge-base/article-versions/{versionId}
POST /api/v1/organizations/{organizationId}/knowledge-base/article-versions/{versionId}/publish-placeholder
```

## Categories and Collections

```text
GET /api/v1/organizations/{organizationId}/knowledge-base/categories
POST /api/v1/organizations/{organizationId}/knowledge-base/categories
GET /api/v1/organizations/{organizationId}/knowledge-base/categories/{categoryId}
PATCH /api/v1/organizations/{organizationId}/knowledge-base/categories/{categoryId}

GET /api/v1/organizations/{organizationId}/knowledge-base/collections
POST /api/v1/organizations/{organizationId}/knowledge-base/collections
GET /api/v1/organizations/{organizationId}/knowledge-base/collections/{collectionId}
PATCH /api/v1/organizations/{organizationId}/knowledge-base/collections/{collectionId}
```

## Visibility Rules

```text
GET /api/v1/organizations/{organizationId}/knowledge-base/visibility-rules
POST /api/v1/organizations/{organizationId}/knowledge-base/articles/{articleId}/visibility-rules
GET /api/v1/organizations/{organizationId}/knowledge-base/visibility-rules/{ruleId}
PATCH /api/v1/organizations/{organizationId}/knowledge-base/visibility-rules/{ruleId}
```

## Search Linkage

```text
GET /api/v1/organizations/{organizationId}/knowledge-base/search-index-links
POST /api/v1/organizations/{organizationId}/knowledge-base/articles/{articleId}/search-index-link-placeholder
POST /api/v1/organizations/{organizationId}/knowledge-base/articles/{articleId}/reindex-placeholder
```

## Feedback

```text
GET /api/v1/organizations/{organizationId}/knowledge-base/feedback
POST /api/v1/organizations/{organizationId}/knowledge-base/articles/{articleId}/feedback
GET /api/v1/organizations/{organizationId}/knowledge-base/feedback/{feedbackId}
PATCH /api/v1/organizations/{organizationId}/knowledge-base/feedback/{feedbackId}
POST /api/v1/organizations/{organizationId}/knowledge-base/feedback/{feedbackId}/mark-reviewed-placeholder
```

## Support Escalation Links

```text
GET /api/v1/organizations/{organizationId}/knowledge-base/support-escalation-links
POST /api/v1/organizations/{organizationId}/knowledge-base/articles/{articleId}/support-escalation-placeholder
GET /api/v1/organizations/{organizationId}/knowledge-base/support-escalation-links/{linkId}
```

## Related Content Links

```text
GET /api/v1/organizations/{organizationId}/knowledge-base/related-content-links
POST /api/v1/organizations/{organizationId}/knowledge-base/articles/{articleId}/related-content-links
PATCH /api/v1/organizations/{organizationId}/knowledge-base/related-content-links/{linkId}
```

## Help Audit History

```text
GET /api/v1/organizations/{organizationId}/knowledge-base/audit-history
GET /api/v1/organizations/{organizationId}/knowledge-base/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership where applicable
- knowledge base feature availability
- required permission
- article visibility rule permits actor access
- linked document/media reference is visible to actor
- related policy/training links respect policy permissions
- support escalation link belongs to same organization where applicable

Private help articles must not cross organization boundaries.

## Status Direction

Suggested article statuses:

```text
draft
in_review_placeholder
published_placeholder
outdated_placeholder
retired_placeholder
manual_review
```

Suggested feedback statuses:

```text
open
reviewed_placeholder
accepted_placeholder
rejected_placeholder
converted_to_support_case_placeholder
manual_review
```

Suggested visibility statuses:

```text
active
inactive
restricted_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- article created or updated
- article version created or published placeholder
- article retired placeholder
- category/collection changed
- visibility rule changed
- feedback reviewed placeholder
- support escalation link created
- related content link changed

## Error Direction

Use standard response shape.

Expected error areas:

- article not found
- article version not found
- category not found
- collection not found
- visibility rule not found
- article visibility denied
- linked document permission denied
- feedback not found
- support escalation link not found
- knowledge base permission denied
- organization not found

## Final Rule

Knowledge base APIs must be organization-aware, visibility-aware, search-aware, support-linked, permission-protected, and auditable.
