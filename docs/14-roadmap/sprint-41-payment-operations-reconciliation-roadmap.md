# Sprint 41 Payment Operations Reconciliation Roadmap

## Purpose

This document defines Sprint 41 for Payment Provider Operations and Reconciliation foundation.

Sprint 41 should create a reusable payment operations layer for provider metadata, payment attempts, callback events, payment status history, reconciliation jobs, mismatches, settlement placeholders, and refund/dispute placeholders.

## Sprint Goal

Enable platform operators and permitted organization admins to track payment attempts, provider references, callback events, status changes, reconciliation jobs, mismatches, and settlement placeholders across billing, SMS packages, commerce, finance, income/giving, and marketplace order workflows.

## Why This Sprint

The platform now has several payment-related domains:

- subscription billing and invoices
- SMS package purchases
- commerce/POS sales
- marketplace orders
- income/giving records
- finance reporting
- future marketplace commissions
- future delivery fees

Without a shared payment operations foundation, payment status can drift between provider callbacks, billing records, wallet credits, order states, and finance records.

## Work Package 1: Payment Provider Metadata

Prepare provider metadata with:

- provider key
- country/region placeholder
- currency support placeholder
- payment method type
- status
- configuration reference placeholder

## Work Package 2: Payment Attempts

Prepare payment attempt records with:

- organization
- amount
- currency
- provider key
- payment method placeholder
- source domain reference
- status
- provider reference placeholder

## Work Package 3: Callback/Webhook Event Metadata

Prepare event metadata with:

- provider key
- provider reference
- event type
- received at
- processing status
- safe payload summary placeholder

## Work Package 4: Payment Status History

Prepare payment status history for:

- initiated
- pending
- successful
- failed
- cancelled
- reversed placeholder
- manual review

## Work Package 5: Reconciliation Jobs

Prepare reconciliation job records with:

- provider key
- period start/end
- status
- started by
- summary counts
- mismatch count

## Work Package 6: Reconciliation Mismatches

Prepare mismatch records for:

- provider successful but platform pending
- platform successful but provider missing
- amount mismatch
- currency mismatch
- duplicate provider reference
- source record missing
- manual review

## Work Package 7: Settlement, Refund, and Dispute Placeholders

Prepare placeholders for:

- settlement batch
- refund request
- dispute/chargeback placeholder
- settlement note
- manual review

## Work Package 8: Flutter Foundation

Prepare screens or placeholders for:

- payment operations dashboard
- payment attempt list
- payment attempt detail
- callback event list
- reconciliation job list
- mismatch review
- settlement placeholder
- refund/dispute placeholder

## Out of Scope

Do not implement:

- direct payment provider SDK calls inside domain modules
- real provider credential management UI
- automatic refunds
- chargeback automation
- full accounting settlement ledger
- cross-organization payment visibility
- tax calculation engine

## Completion Standard

Sprint 41 is complete when payment attempts, callback events, status history, reconciliation jobs, mismatches, and settlement/refund placeholders are available through provider-agnostic, permission-protected, and auditable foundations.

## Final Rule

Payment operations must keep payment state explainable across domains without leaking provider-specific behavior into business modules.
