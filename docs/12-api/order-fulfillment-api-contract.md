# Order Fulfillment API Contract

## Purpose

This document defines the MVP API contract for Order Fulfillment and Pickup foundation.

The API should support fulfillment queue, status updates, preparation checklist, pickup readiness, cancellation, and reconciliation while preserving Marketplace order boundaries.

## Owner

```text
Commerce Domain
Notification Engine where customer updates are sent
```

## Base Path

```text
/api/v1/organizations/{organizationId}/commerce/fulfillment
```

## Fulfillment Queue

```text
GET /api/v1/organizations/{organizationId}/commerce/fulfillment/orders
```

### Query Parameters

```text
status
paymentStatus
createdFrom
createdTo
page
perPage
```

## Fulfillment Detail

```text
GET /api/v1/organizations/{organizationId}/commerce/fulfillment/orders/{orderId}
```

## Accept Order

```text
POST /api/v1/organizations/{organizationId}/commerce/fulfillment/orders/{orderId}/accept
```

### Request Direction

```json
{
  "notes": "Order accepted for preparation"
}
```

## Preparation Checklist

```text
GET /api/v1/organizations/{organizationId}/commerce/fulfillment/orders/{orderId}/checklist
PATCH /api/v1/organizations/{organizationId}/commerce/fulfillment/orders/{orderId}/checklist/{itemId}
```

### Update Checklist Item

```json
{
  "preparedQuantity": 2,
  "status": "prepared",
  "notes": "Packed"
}
```

## Mark Ready For Pickup

```text
POST /api/v1/organizations/{organizationId}/commerce/fulfillment/orders/{orderId}/ready-for-pickup
```

### Request Direction

```json
{
  "pickupLocationLabel": "Main desk",
  "pickupInstructions": "Show order number at pickup",
  "notes": "Ready"
}
```

## Complete Order

```text
POST /api/v1/organizations/{organizationId}/commerce/fulfillment/orders/{orderId}/complete
```

### Request Direction

```json
{
  "notes": "Customer collected order"
}
```

## Cancel Order

```text
POST /api/v1/organizations/{organizationId}/commerce/fulfillment/orders/{orderId}/cancel
```

### Request Direction

```json
{
  "reason": "Customer cancelled",
  "releaseReservedStock": true,
  "notes": "Optional note"
}
```

## Fulfillment History

```text
GET /api/v1/organizations/{organizationId}/commerce/fulfillment/orders/{orderId}/history
```

## Customer Status View

```text
GET /api/v1/storefronts/{organizationSlug}/orders/{orderId}/status
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "order": {
      "id": "...",
      "orderNumber": "ORD-0001",
      "status": "ready_for_pickup",
      "paymentStatus": "paid",
      "pickupLocationLabel": "Main desk"
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Fulfillment Reconciliation Review

```text
GET /api/v1/system/commerce/fulfillment-reconciliation-items
POST /api/v1/system/commerce/fulfillment-reconciliation-items/{itemId}/resolve
```

## Required Checks

Organization routes should check:

- authenticated user
- organization membership
- Commerce/Marketplace module availability
- required permission
- feature availability
- organization ownership

Customer status routes should expose only safe customer-facing fields.

System/admin routes should check:

- authenticated platform admin or support role
- required system permission
- audit logging

## Status Direction

Suggested fulfillment statuses:

```text
pending_review
accepted
preparing
ready_for_pickup
completed
cancelled
manual_review
```

Suggested checklist statuses:

```text
pending
prepared
unavailable
replaced_placeholder
cancelled
```

## Audit Direction

Audit should be written for:

- order accepted
- checklist updated
- order marked ready for pickup
- order completed
- order cancelled
- stock released or deducted where supported
- notification triggered
- reconciliation resolved

## Error Direction

Use standard response shape.

Expected error areas:

- order not found
- order not eligible for fulfillment
- invalid status transition
- checklist item not found
- stock release failed
- permission denied
- organization not found

## Final Rule

Fulfillment APIs must preserve order ownership, safe customer visibility, permission checks, auditability, and clear status transitions.
