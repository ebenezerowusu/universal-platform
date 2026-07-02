# Delivery Logistics UI Contract

## Purpose

This document defines the MVP UI contract for Delivery and Logistics foundation.

The screens should help organization users configure delivery zones, fee rules, delivery requests, status updates, and proof-of-delivery placeholders without introducing full rider dispatch too early.

## Navigation Direction

Delivery screens should appear under Commerce when:

- user is authenticated
- organization is selected
- Commerce/Delivery feature is available
- user has required permission
- feature availability allows delivery

## Delivery Dashboard

Should show:

- requested deliveries
- accepted deliveries
- assigned placeholder deliveries
- delivered deliveries for selected period
- failed or manual review count where available

## Delivery Zones Screen

Should show:

- zone name
- area label
- status
- linked fee rule where available
- edit action where permitted

## Fee Rules Screen

Should show:

- zone
- rule type
- amount
- currency
- status
- edit action where permitted

## Delivery Request Detail

Should show:

- order number
- customer safe label
- delivery address summary
- zone
- delivery fee
- delivery status
- notes/history where available

## Assignment Placeholder Screen

Should support:

- assigned label
- optional note
- confirmation action

This is only a placeholder and should not imply full rider dispatch.

## Status Update Screen

Should support:

- status selection
- optional note
- confirmation action

## Proof of Delivery Placeholder

Should support:

- confirmation note
- delivered timestamp
- future file/image placeholder where supported later

## Customer Delivery Status View

Customer-facing delivery status should show only safe fields:

- delivery status
- area label where safe
- updated time
- next steps text

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 26:

- live rider tracking screens
- route optimization screens
- driver payout screens
- ride-hailing screens
- third-party logistics setup screens
- advanced returns logistics screens

## Final Rule

Delivery UI should make basic delivery operations clear without overpromising live dispatch or ride-hailing behavior.
