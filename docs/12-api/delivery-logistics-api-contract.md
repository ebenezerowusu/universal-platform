# Delivery Logistics API Contract

## Purpose

This document defines the MVP API contract for Delivery and Logistics foundation.

The API should support delivery zones, fee rules, delivery requests, status updates, proof-of-delivery placeholders, customer delivery status, and reconciliation while preserving order ownership.

## Owner

```text
Commerce Delivery Subdomain
Notification Engine where customer updates are sent
```

## Base Path

```text
/api/v1/organizations/{organizationId}/commerce/delivery
```

## Delivery Zones

```text
GET /api/v1/organizations/{organizationId}/commerce/delivery/zones
POST /api/v1/organizations/{organizationId}/commerce/delivery/zones
GET /api/v1/organizations/{organizationId}/commerce/delivery/zones/{zoneId}
PATCH /api/v1/organizations/{organizationId}/commerce/delivery/zones/{zoneId}
```

### Create Zone Request

```json
{
  "name": "Kumasi Central",
  "areaLabel": "Central area",
  "status": "active"
}
```

## Delivery Fee Rules

```text
GET /api/v1/organizations/{organizationId}/commerce/delivery/fee-rules
POST /api/v1/organizations/{organizationId}/commerce/delivery/fee-rules
GET /api/v1/organizations/{organizationId}/commerce/delivery/fee-rules/{feeRuleId}
PATCH /api/v1/organizations/{organizationId}/commerce/delivery/fee-rules/{feeRuleId}
```

### Create Fee Rule Request

```json
{
  "zoneId": "...",
  "type": "flat_fee",
  "amount": 20,
  "currency": "GHS",
  "status": "active"
}
```

## Delivery Quote Placeholder

```text
POST /api/v1/organizations/{organizationId}/commerce/delivery/quote
```

### Request Direction

```json
{
  "zoneId": "...",
  "orderId": "..."
}
```

## Delivery Request

```text
GET /api/v1/organizations/{organizationId}/commerce/delivery/requests
POST /api/v1/organizations/{organizationId}/commerce/delivery/requests
GET /api/v1/organizations/{organizationId}/commerce/delivery/requests/{deliveryRequestId}
```

### Create Delivery Request

```json
{
  "orderId": "...",
  "zoneId": "...",
  "address": {
    "recipientName": "Optional name",
    "phone": "Optional phone",
    "addressLine": "House 10, Street Name",
    "landmark": "Near station",
    "areaLabel": "Kumasi Central",
    "notes": "Call before arrival"
  }
}
```

## Assignment Placeholder

```text
POST /api/v1/organizations/{organizationId}/commerce/delivery/requests/{deliveryRequestId}/assign
```

### Request Direction

```json
{
  "assignedToLabel": "Internal dispatch desk",
  "notes": "Assignment placeholder"
}
```

## Status Update

```text
POST /api/v1/organizations/{organizationId}/commerce/delivery/requests/{deliveryRequestId}/status
```

### Request Direction

```json
{
  "status": "delivered",
  "notes": "Delivered to recipient"
}
```

## Proof of Delivery Placeholder

```text
POST /api/v1/organizations/{organizationId}/commerce/delivery/requests/{deliveryRequestId}/proof
```

### Request Direction

```json
{
  "confirmationNote": "Recipient confirmed delivery",
  "deliveredAt": "2026-07-02T10:00:00Z"
}
```

## Customer Delivery Status

```text
GET /api/v1/storefronts/{organizationSlug}/orders/{orderId}/delivery-status
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "delivery": {
      "status": "delivered",
      "areaLabel": "Kumasi Central",
      "updatedAt": "..."
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Delivery Reconciliation Review

```text
GET /api/v1/system/commerce/delivery-reconciliation-items
POST /api/v1/system/commerce/delivery-reconciliation-items/{itemId}/resolve
```

## Required Checks

Organization routes should check:

- authenticated user
- organization membership
- Commerce/Delivery feature availability
- required permission
- organization ownership

Customer status routes should expose only safe customer-facing fields.

System/admin routes should check:

- authenticated platform admin or support role
- required system permission
- audit logging

## Status Direction

Suggested delivery statuses:

```text
requested
accepted
preparing_for_dispatch
assigned_placeholder
out_for_delivery_placeholder
delivered
failed
cancelled
manual_review
```

## Audit Direction

Audit should be written for:

- delivery zone created or updated
- delivery fee rule created or updated
- delivery request created
- delivery assigned where supported
- delivery status changed
- proof-of-delivery added
- delivery failed or cancelled
- reconciliation resolved

## Error Direction

Use standard response shape.

Expected error areas:

- delivery feature unavailable
- zone not found
- fee rule not found
- order not found
- order not eligible for delivery
- delivery request not found
- invalid delivery status transition
- permission denied
- organization not found

## Final Rule

Delivery APIs must preserve order ownership, safe customer visibility, permission checks, auditability, and clear delivery status transitions.
