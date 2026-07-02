# Sprint 26 Delivery Logistics Review

## Purpose

This document defines review checks for Sprint 26.

Sprint 26 implements Delivery and Logistics foundation.

## Review 1: Scope

Sprint 26 may include:

- delivery feature registration foundation
- delivery zone foundation
- delivery fee rule foundation
- customer delivery address foundation
- delivery request foundation linked to order
- delivery assignment placeholder where practical
- delivery status foundation
- proof-of-delivery placeholder where practical
- customer notification trigger foundation where practical
- permission and feature checks
- audit records for important delivery actions
- API and UI foundations

Sprint 26 should not include:

- live rider tracking
- route optimization
- driver payout management
- ride-hailing workflows
- third-party logistics integrations
- marketplace commission settlement
- advanced return/refund logistics
- direct payment provider calls inside Logistics or Commerce Domain
- unrelated POS expansion

## Review 2: Domain Boundaries

Confirm:

- delivery workflows are owned by Commerce Delivery or Logistics subdomain
- Commerce Domain owns the order being delivered
- Notification Engine handles customer updates where used
- Platform Core does not contain delivery-specific business rules

## Review 3: Zone and Fee Rules

Confirm:

- delivery zones are organization-owned
- fee rules are organization-owned
- fee currency is explicit
- delivery fee changes are auditable
- customer-facing fee fields are safe

## Review 4: Delivery Request Rules

Confirm:

- delivery request is linked to an eligible order
- customer address data is protected
- status transitions are controlled
- assignment placeholder does not imply live dispatch
- proof-of-delivery placeholder is privacy-aware

## Review 5: Customer Visibility

Confirm customer-facing delivery status exposes only safe fields:

- status
- area label where safe
- updated time
- next steps text

Do not expose internal notes, staff labels, or operational details.

## Review 6: Reconciliation

Confirm:

- delivery request without eligible order can be reviewed
- missing delivery fee can be reviewed where required
- delivered without proof can be reviewed where policy requires proof
- failed/cancelled delivery reason is captured where practical
- manual review actions are auditable

## Review 7: API Contract

Confirm implementation aligns with:

```text
docs/12-api/delivery-logistics-api-contract.md
```

## Review 8: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/delivery-logistics-ui-contract.md
```

## Review 9: Test Coverage

Tests should cover where practical:

- delivery zone create/update
- fee rule create/update
- quote placeholder behavior
- delivery request creation
- invalid order eligibility
- delivery status update
- invalid status transition
- proof-of-delivery placeholder
- permission denied behavior
- reconciliation item behavior

## Final Rule

Sprint 26 is complete when organizations can configure basic delivery, create delivery requests from orders, update delivery status, and reconcile delivery activity without building full ride-hailing or live dispatch operations.
