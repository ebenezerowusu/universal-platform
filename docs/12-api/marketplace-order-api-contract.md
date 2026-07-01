# Marketplace Order API Contract

## Purpose

This document defines the MVP API contract for Marketplace and Order foundation.

The API should support storefront catalog, cart, checkout intent, order creation, order status, and order management while reusing Commerce/POS product and inventory foundations.

## Owner

```text
Commerce Domain
Payment Engine where online payment is used
```

## Storefront Base Path

```text
/api/v1/storefronts/{organizationSlug}
```

## Organization Admin Base Path

```text
/api/v1/organizations/{organizationId}/commerce
```

## Storefront Catalog

```text
GET /api/v1/storefronts/{organizationSlug}/products
GET /api/v1/storefronts/{organizationSlug}/products/{productId}
```

### Catalog Rules

Only show products that are:

- organization-owned
- active
- public storefront visible
- safe for customer display

## Cart

```text
POST /api/v1/storefronts/{organizationSlug}/cart
GET /api/v1/storefronts/{organizationSlug}/cart/{cartId}
PATCH /api/v1/storefronts/{organizationSlug}/cart/{cartId}
```

### Create Cart Request

```json
{
  "items": [
    {
      "productId": "...",
      "quantity": 2
    }
  ]
}
```

## Checkout Intent

```text
POST /api/v1/storefronts/{organizationSlug}/cart/{cartId}/checkout-intent
```

### Request Direction

```json
{
  "customerName": "Optional name",
  "customerPhone": "Optional phone",
  "customerEmail": "Optional email",
  "paymentMode": "cash_on_pickup"
}
```

### Direction

If online payment is selected, the checkout flow should call Payment Engine.

Commerce Domain must not call payment providers directly.

## Orders

```text
POST /api/v1/storefronts/{organizationSlug}/orders
GET /api/v1/storefronts/{organizationSlug}/orders/{orderId}
```

### Create Order Request

```json
{
  "cartId": "...",
  "checkoutIntentId": "...",
  "customerReference": "optional",
  "notes": "Optional order note"
}
```

## Admin Order Management

```text
GET /api/v1/organizations/{organizationId}/commerce/orders
GET /api/v1/organizations/{organizationId}/commerce/orders/{orderId}
PATCH /api/v1/organizations/{organizationId}/commerce/orders/{orderId}
```

### Update Order Request Direction

```json
{
  "status": "confirmed",
  "notes": "Order confirmed for pickup"
}
```

## Order Confirmation

```text
GET /api/v1/storefronts/{organizationSlug}/orders/{orderId}/confirmation
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "order": {
      "id": "...",
      "orderNumber": "ORD-0001",
      "status": "confirmed",
      "paymentStatus": "pending",
      "totalAmount": 40,
      "currency": "GHS"
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Order Reconciliation Review

```text
GET /api/v1/system/commerce/order-reconciliation-items
POST /api/v1/system/commerce/order-reconciliation-items/{itemId}/resolve
```

## Required Checks

Storefront routes should check:

- organization exists
- storefront feature is available
- product visibility rules
- safe customer-facing fields only

Admin routes should check:

- authenticated user
- organization membership
- Commerce/Marketplace module availability
- required permission
- feature availability
- organization ownership

System/admin routes should check:

- authenticated platform admin or support role
- required system permission
- audit logging

## Status Direction

Suggested cart statuses:

```text
active
checked_out
expired
cancelled
```

Suggested order statuses:

```text
pending
confirmed
ready_for_pickup_placeholder
completed
cancelled
manual_review
```

Suggested payment statuses:

```text
not_required
pending
processing
paid
failed
manual_review
```

## Inventory Direction

Where inventory tracking is enabled:

- check available stock during checkout or order confirmation
- reserve stock where supported
- deduct stock according to organization policy
- release reservation when order expires or is cancelled

## Audit Direction

Audit should be written for:

- product visibility changed
- order created
- checkout initiated
- payment status changed
- order status changed
- stock reserved or released where supported
- reconciliation resolved

## Error Direction

Use standard response shape.

Expected error areas:

- storefront unavailable
- product unavailable
- insufficient stock
- cart not found
- order not found
- checkout failed
- payment initiation failed
- permission denied
- organization not found

## Final Rule

Marketplace APIs must reuse Commerce/POS product, inventory, and payment foundations. Do not duplicate product logic for storefront flows.
