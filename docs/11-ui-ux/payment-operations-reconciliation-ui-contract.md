# Payment Operations Reconciliation UI Contract

## Purpose

This document defines the MVP UI contract for Payment Provider Operations and Reconciliation foundation.

The screens should help permitted users review payment attempts, provider metadata, callback events, status history, reconciliation jobs, mismatches, settlement placeholders, and refund/dispute placeholders.

## Navigation Direction

Payment operations screens should appear when:

- user is authenticated
- organization is selected
- payment operations feature is available
- user has required permission
- feature availability allows payment operations visibility

## Payment Operations Dashboard

Should show:

- successful payments
- pending payments
- failed payments
- open mismatch count
- last reconciliation status
- provider warning count where available

## Provider List

Should show:

- provider name/key
- country/region placeholder
- supported currency placeholder
- payment method type
- status
- action to view detail where permitted

## Payment Attempt List

Should show:

- amount
- currency
- provider
- source domain
- status
- provider reference placeholder
- created date
- action to view detail

## Payment Attempt Detail

Should show:

- payment metadata
- source reference safe label where allowed
- provider reference placeholder
- current status
- status history
- callback events where available
- reconciliation status placeholder

## Callback Event List

Should show:

- provider
- provider reference
- event type
- processing status
- received date
- action to view detail

## Reconciliation Job List

Should show:

- provider
- period
- status
- started by safe label
- summary counts
- mismatch count
- action to view detail

## Mismatch Review

Should show:

- mismatch type
- provider reference
- platform payment reference
- source domain
- severity placeholder
- status
- review action where permitted

## Settlement Placeholder

Should show:

- settlement batch key
- provider
- period
- amount placeholder
- status
- notes placeholder

## Refund and Dispute Placeholder

Should show:

- request type
- payment reference
- amount placeholder
- reason
- status
- created by safe label
- created date

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 41:

- real provider credential management UI
- automatic refund UI
- chargeback automation UI
- full accounting settlement ledger UI
- tax calculation UI
- provider-specific checkout UI

## Final Rule

Payment operations UI should make payment state and reconciliation issues visible without exposing provider secrets or bypassing Payment Engine boundaries.
