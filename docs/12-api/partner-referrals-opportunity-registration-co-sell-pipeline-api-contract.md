# Partner Referrals Opportunity Registration Co-Sell Pipeline API Contract

## Purpose

This document defines the MVP API contract for Partner Referrals, Opportunity Registration, and Co-Sell Pipeline foundation.

The API should support partner referral dashboards, referral leads, opportunity registrations, co-sell pipeline stages, eligibility/readiness reviews, duplicate/conflict reviews, revenue/subscription links, customer success/implementation handoffs, partner status updates, and referral audit history while preserving partner and tenant boundaries.

## Owner

```text
Partner Referral Foundation
Permission Engine
Audit Engine
Partner Ecosystem Foundation
Partner Enablement Foundation
Subscription Billing/Revenue Operations Foundation
Customer Success Foundation
Implementation Delivery Foundation
Solution Blueprint Foundation
Notification and Workflow Foundations where reviews/handoffs are consumed later
```

## Base Path

```text
/api/v1/organizations/{organizationId}/partner-referrals
```

Platform-level partner referral administration may later use stricter platform-admin routes.

## Partner Referral Dashboard

```text
GET /api/v1/organizations/{organizationId}/partner-referrals/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "openReferrals": 18,
    "registeredOpportunities": 9,
    "openConflictReviews": 3,
    "activeCoSellItems": 5,
    "handoffsPending": 4
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Referral Leads

```text
GET /api/v1/organizations/{organizationId}/partner-referrals/referral-leads
POST /api/v1/organizations/{organizationId}/partner-referrals/referral-leads
GET /api/v1/organizations/{organizationId}/partner-referrals/referral-leads/{referralId}
PATCH /api/v1/organizations/{organizationId}/partner-referrals/referral-leads/{referralId}
POST /api/v1/organizations/{organizationId}/partner-referrals/referral-leads/{referralId}/close-placeholder
```

## Opportunity Registrations

```text
GET /api/v1/organizations/{organizationId}/partner-referrals/opportunities
POST /api/v1/organizations/{organizationId}/partner-referrals/referral-leads/{referralId}/opportunities
GET /api/v1/organizations/{organizationId}/partner-referrals/opportunities/{opportunityId}
PATCH /api/v1/organizations/{organizationId}/partner-referrals/opportunities/{opportunityId}
POST /api/v1/organizations/{organizationId}/partner-referrals/opportunities/{opportunityId}/mark-reviewed-placeholder
```

## Co-Sell Pipeline Stages

```text
GET /api/v1/organizations/{organizationId}/partner-referrals/pipeline-stages
POST /api/v1/organizations/{organizationId}/partner-referrals/opportunities/{opportunityId}/pipeline-stages
GET /api/v1/organizations/{organizationId}/partner-referrals/pipeline-stages/{stageId}
PATCH /api/v1/organizations/{organizationId}/partner-referrals/pipeline-stages/{stageId}
```

## Eligibility and Readiness Reviews

```text
GET /api/v1/organizations/{organizationId}/partner-referrals/eligibility-reviews
POST /api/v1/organizations/{organizationId}/partner-referrals/opportunities/{opportunityId}/eligibility-review-placeholder
GET /api/v1/organizations/{organizationId}/partner-referrals/eligibility-reviews/{reviewId}
POST /api/v1/organizations/{organizationId}/partner-referrals/eligibility-reviews/{reviewId}/mark-reviewed-placeholder
```

## Duplicate and Conflict Reviews

```text
GET /api/v1/organizations/{organizationId}/partner-referrals/conflict-reviews
POST /api/v1/organizations/{organizationId}/partner-referrals/opportunities/{opportunityId}/conflict-review-placeholder
GET /api/v1/organizations/{organizationId}/partner-referrals/conflict-reviews/{reviewId}
PATCH /api/v1/organizations/{organizationId}/partner-referrals/conflict-reviews/{reviewId}
POST /api/v1/organizations/{organizationId}/partner-referrals/conflict-reviews/{reviewId}/resolve-placeholder
```

## Revenue and Subscription Links

```text
GET /api/v1/organizations/{organizationId}/partner-referrals/revenue-subscription-links
POST /api/v1/organizations/{organizationId}/partner-referrals/revenue-subscription-links
PATCH /api/v1/organizations/{organizationId}/partner-referrals/revenue-subscription-links/{linkId}
```

## Customer Success and Implementation Handoffs

```text
GET /api/v1/organizations/{organizationId}/partner-referrals/handoffs
POST /api/v1/organizations/{organizationId}/partner-referrals/handoffs
GET /api/v1/organizations/{organizationId}/partner-referrals/handoffs/{handoffId}
PATCH /api/v1/organizations/{organizationId}/partner-referrals/handoffs/{handoffId}
POST /api/v1/organizations/{organizationId}/partner-referrals/handoffs/{handoffId}/mark-completed-placeholder
```

## Partner Status Updates

```text
GET /api/v1/organizations/{organizationId}/partner-referrals/status-updates
POST /api/v1/organizations/{organizationId}/partner-referrals/opportunities/{opportunityId}/status-updates
GET /api/v1/organizations/{organizationId}/partner-referrals/status-updates/{statusUpdateId}
POST /api/v1/organizations/{organizationId}/partner-referrals/status-updates/{statusUpdateId}/send-placeholder
```

## Referral Audit History

```text
GET /api/v1/organizations/{organizationId}/partner-referrals/audit-history
GET /api/v1/organizations/{organizationId}/partner-referrals/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership where applicable
- partner referral feature availability
- required permission
- partner active status and readiness where required
- linked revenue/subscription records are visible to actor
- partner-visible status updates are safe summaries only
- opportunity records do not imply committed revenue
- conflict reviews are audit logged

Referral records must not expose unauthorized tenant, prospect, or partner details.

## Status Direction

Suggested referral statuses:

```text
submitted
under_review_placeholder
accepted_placeholder
converted_to_opportunity_placeholder
closed_placeholder
manual_review
```

Suggested opportunity statuses:

```text
registered
reviewing_placeholder
qualified_placeholder
co_sell_placeholder
converted_placeholder
closed_placeholder
manual_review
```

Suggested conflict statuses:

```text
none_detected_placeholder
open_review
resolved_placeholder
blocked_placeholder
manual_review
```

Suggested handoff statuses:

```text
pending_placeholder
accepted_placeholder
in_progress_placeholder
completed_placeholder
cancelled_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- referral submitted
- referral updated or closed
- opportunity registered or updated
- pipeline stage changed
- eligibility/readiness reviewed placeholder
- duplicate/conflict review recorded placeholder
- revenue/subscription link changed
- customer success/implementation handoff created
- partner status update sent placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- referral lead not found
- opportunity not found
- pipeline stage not found
- eligibility review not found
- conflict review not found
- related revenue/subscription link not found
- handoff not found
- status update not found
- partner referral permission denied
- organization not found

## Final Rule

Partner referral APIs must be partner-safe, tenant-safe, revenue-boundary-aware, permission-protected, review-based, and auditable.
