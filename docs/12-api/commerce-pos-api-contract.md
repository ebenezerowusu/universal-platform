# Commerce POS API Contract

## Purpose

This document defines the MVP API contract for Commerce and POS foundation.

The API should support product categories, product catalog, inventory movement, POS sales, receipts, and sales reconciliation while preserving organization boundaries.

## Owner

```text
Commerce Domain
Payment Engine where online payment is used
```

## Base Path

```text
/api/v1/organizations/{organizationId}/commerce
```

## Product Categories

```text
GET /api/v1/organizations/{organizationId}/commerce/product-categories
POST /api/v1/organizations/{organizationId}/commerce/product-categories
GET /api/v1/organizations/{organizationId}/commerce/product-categories/{categoryId}
PATCH /api/v1/organizations/{organizationId}/commerce/product-categories/{categoryId}
```

### Create Category Request

```json
{
  "name": "Books",
  "code": "books",
  "status": "active"
}
```

## Products

```text
GET /api/v1/organizations/{organizationId}/commerce/products
POST /api/v1/organizations/{organizationId}/commerce/products
GET /api/v1/organizations/{organizationId}/commerce/products/{productId}
PATCH /api/v1/organizations/{organizationId}/commerce/products/{productId}
```

### Create Product Request

```json
{
  "categoryId": "...",
  "name": "Notebook",
  "sku": "NB-001",
  "barcode": "optional",
  "price": 20,
  "currency": "GHS",
  "trackInventory": true,
  "status": "active"
}
```

## Stock Items

```text
GET /api/v1/organizations/{organizationId}/commerce/stock-items
GET /api/v1/organizations/{organizationId}/commerce/products/{productId}/stock
```

## Inventory Movements

```text
GET /api/v1/organizations/{organizationId}/commerce/inventory-movements
POST /api/v1/organizations/{organizationId}/commerce/inventory-movements
```

### Create Movement Request

```json
{
  "productId": "...",
  "type": "adjustment",
  "quantity": 10,
  "reason": "Opening stock",
  "notes": "Optional note"
}
```

## POS Sales

```text
GET /api/v1/organizations/{organizationId}/commerce/pos/sales
POST /api/v1/organizations/{organizationId}/commerce/pos/sales
GET /api/v1/organizations/{organizationId}/commerce/pos/sales/{saleId}
```

### Create Sale Request

```json
{
  "items": [
    {
      "productId": "...",
      "quantity": 2,
      "unitPrice": 20
    }
  ],
  "paymentMode": "cash",
  "notes": "Optional note"
}
```

## Payment Link

```text
POST /api/v1/organizations/{organizationId}/commerce/pos/sales/{saleId}/initiate-payment
```

### Direction

This route should call Payment Engine where online payment is supported.

Commerce Domain must not call payment providers directly.

## Receipts

```text
GET /api/v1/organizations/{organizationId}/commerce/receipts
GET /api/v1/organizations/{organizationId}/commerce/receipts/{receiptId}
```

### Receipt Response Direction

```json
{
  "success": true,
  "data": {
    "receipt": {
      "id": "...",
      "receiptNumber": "POS-0001",
      "saleId": "...",
      "totalAmount": 40,
      "currency": "GHS",
      "issuedAt": "..."
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Sales Reconciliation Review

```text
GET /api/v1/system/commerce/sales-reconciliation-items
POST /api/v1/system/commerce/sales-reconciliation-items/{itemId}/resolve
```

## Required Checks

Organization routes should check:

- authenticated user
- organization membership
- Commerce/POS module availability
- required permission
- feature availability
- organization ownership

System/admin routes should check:

- authenticated platform admin or support role
- required system permission
- audit logging

## Status Direction

Suggested sale statuses:

```text
draft
confirmed
paid
cancelled
refunded_placeholder
manual_review
```

Suggested inventory movement types:

```text
opening
adjustment
sale_deduction
return_placeholder
damage_placeholder
```

## Audit Direction

Audit should be written for:

- product created or updated
- product price changed
- stock adjusted
- sale created
- sale cancelled where supported
- payment status changed
- receipt issued
- reconciliation resolved

## Error Direction

Use standard response shape.

Expected error areas:

- product not found
- product inactive
- insufficient stock
- sale not found
- payment initiation failed
- duplicate receipt prevented
- permission denied
- organization not found

## Final Rule

Commerce APIs must preserve organization ownership, inventory rules, and Payment Engine boundaries. No payment provider logic should appear in Commerce Domain routes.
