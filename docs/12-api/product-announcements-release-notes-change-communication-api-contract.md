# Product Announcements Release Notes Change Communication API Contract

## Purpose

This document defines the MVP API contract for Product Announcements, Release Notes, and Change Communication foundation.

The API should support communication dashboards, announcements, release notes, change notices, maintenance notices, audience targets, read receipts, related links, delivery status, and communication audit history while preserving visibility boundaries.

## Owner

```text
Product Communication Foundation
Permission Engine
Audit Engine
Notification Center Foundation
Configuration/Feature Flag Foundation
Knowledge Base Foundation
Support Operations Foundation
Monitoring/Incident Foundation
```

## Base Path

```text
/api/v1/organizations/{organizationId}/product-communication
```

Platform-level announcements may later use stricter platform-admin routes.

## Communication Dashboard

```text
GET /api/v1/organizations/{organizationId}/product-communication/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "activeAnnouncements": 4,
    "publishedReleaseNotes": 12,
    "scheduledMaintenanceNotices": 1,
    "pendingAcknowledgements": 18,
    "deliveryWarnings": 2
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Announcements

```text
GET /api/v1/organizations/{organizationId}/product-communication/announcements
POST /api/v1/organizations/{organizationId}/product-communication/announcements
GET /api/v1/organizations/{organizationId}/product-communication/announcements/{announcementId}
PATCH /api/v1/organizations/{organizationId}/product-communication/announcements/{announcementId}
POST /api/v1/organizations/{organizationId}/product-communication/announcements/{announcementId}/publish-placeholder
POST /api/v1/organizations/{organizationId}/product-communication/announcements/{announcementId}/retire-placeholder
```

## Release Notes

```text
GET /api/v1/organizations/{organizationId}/product-communication/release-notes
POST /api/v1/organizations/{organizationId}/product-communication/release-notes
GET /api/v1/organizations/{organizationId}/product-communication/release-notes/{releaseNoteId}
PATCH /api/v1/organizations/{organizationId}/product-communication/release-notes/{releaseNoteId}
POST /api/v1/organizations/{organizationId}/product-communication/release-notes/{releaseNoteId}/publish-placeholder
```

## Change Notices

```text
GET /api/v1/organizations/{organizationId}/product-communication/change-notices
POST /api/v1/organizations/{organizationId}/product-communication/change-notices
GET /api/v1/organizations/{organizationId}/product-communication/change-notices/{noticeId}
PATCH /api/v1/organizations/{organizationId}/product-communication/change-notices/{noticeId}
POST /api/v1/organizations/{organizationId}/product-communication/change-notices/{noticeId}/publish-placeholder
```

## Maintenance Notices

```text
GET /api/v1/organizations/{organizationId}/product-communication/maintenance-notices
POST /api/v1/organizations/{organizationId}/product-communication/maintenance-notices
GET /api/v1/organizations/{organizationId}/product-communication/maintenance-notices/{noticeId}
PATCH /api/v1/organizations/{organizationId}/product-communication/maintenance-notices/{noticeId}
POST /api/v1/organizations/{organizationId}/product-communication/maintenance-notices/{noticeId}/mark-completed-placeholder
```

## Audience Targets

```text
GET /api/v1/organizations/{organizationId}/product-communication/audience-targets
POST /api/v1/organizations/{organizationId}/product-communication/audience-targets
GET /api/v1/organizations/{organizationId}/product-communication/audience-targets/{targetId}
PATCH /api/v1/organizations/{organizationId}/product-communication/audience-targets/{targetId}
```

## Read Receipts

```text
GET /api/v1/organizations/{organizationId}/product-communication/read-receipts
POST /api/v1/organizations/{organizationId}/product-communication/announcements/{announcementId}/read-placeholder
GET /api/v1/organizations/{organizationId}/product-communication/read-receipts/{receiptId}
```

## Related Links

```text
GET /api/v1/organizations/{organizationId}/product-communication/related-links
POST /api/v1/organizations/{organizationId}/product-communication/related-links
PATCH /api/v1/organizations/{organizationId}/product-communication/related-links/{linkId}
```

## Delivery Status

```text
GET /api/v1/organizations/{organizationId}/product-communication/delivery-status
GET /api/v1/organizations/{organizationId}/product-communication/announcements/{announcementId}/delivery-status
POST /api/v1/organizations/{organizationId}/product-communication/announcements/{announcementId}/queue-delivery-placeholder
```

## Communication Audit History

```text
GET /api/v1/organizations/{organizationId}/product-communication/audit-history
GET /api/v1/organizations/{organizationId}/product-communication/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership where applicable
- product communication feature availability
- required permission
- audience target belongs to allowed organization scope
- delivery respects Notification Center preferences where applicable
- related feature/config/help/support link is visible to actor
- platform-wide messages require platform-admin rules later

Communication records must not cross organization boundaries unless explicitly platform-scoped and permissioned.

## Status Direction

Suggested announcement statuses:

```text
draft
scheduled_placeholder
published_placeholder
retired_placeholder
manual_review
```

Suggested maintenance statuses:

```text
scheduled_placeholder
active_placeholder
completed_placeholder
cancelled_placeholder
manual_review
```

Suggested delivery statuses:

```text
queued_placeholder
delivered_placeholder
partially_delivered_placeholder
failed_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- announcement created or updated
- announcement published placeholder
- release note created or published placeholder
- change notice created or published placeholder
- maintenance notice created or updated
- audience target changed
- read receipt captured placeholder
- related feature/help/support link changed

## Error Direction

Use standard response shape.

Expected error areas:

- announcement not found
- release note not found
- change notice not found
- maintenance notice not found
- audience target not found
- read receipt not found
- delivery status unavailable
- notification preference conflict
- product communication permission denied
- organization not found

## Final Rule

Product communication APIs must be visibility-aware, notification-aware, audience-aware, permission-protected, and auditable.
