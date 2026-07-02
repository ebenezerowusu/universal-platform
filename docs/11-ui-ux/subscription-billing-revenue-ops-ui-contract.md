# Subscription Billing Revenue Operations UI Contract

## Purpose

This document defines the MVP UI contract for Subscription Billing and Revenue Operations foundation.

The screens should help permitted users review plan subscriptions, invoices, billing status, payment references, entitlement history, renewal placeholders, cancellation placeholders, and revenue operation notes.

## Navigation Direction

Billing screens should appear when:

- user is authenticated
- organization is selected
- billing feature is available
- user has required permission
- feature availability allows billing visibility

## Billing Dashboard

Should show:

- current plan
- subscription status
- billing interval placeholder
- next renewal date placeholder
- open invoice count
- payment status placeholder
- entitlement warning count where available

## Plan List

Should show:

- plan title
- plan key
- billing interval placeholder
- currency
- amount placeholder
- status
- action to view detail where permitted

## Subscription Detail

Should show:

- current plan
- subscription status
- current billing period
- trial end placeholder
- cancellation placeholder
- renewal placeholder
- entitlement status summary

## Invoice List

Should show:

- invoice number placeholder
- amount
- currency
- status
- due date
- paid date where available
- action to view detail

## Invoice Detail

Should show:

- invoice metadata
- subscription reference
- payment references
- status history placeholder
- safe notes where available

## Entitlement History

Should show:

- change type
- affected module/feature
- old value placeholder
- new value placeholder
- actor safe label
- timestamp

## Renewal and Cancellation Placeholder

Should show:

- renewal status
- cancellation request status
- effective date placeholder
- grace period placeholder
- restore placeholder where permitted

## Revenue Operation Notes

Should show:

- note type
- safe summary
- related invoice/subscription where available
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

Do not implement in Sprint 40:

- direct payment provider checkout UI
- tax calculation UI
- full accounting ledger UI
- automatic card charging UI
- revenue recognition UI
- external billing provider setup UI

## Final Rule

Billing UI should explain subscription and entitlement status without exposing provider-specific payment behavior or accounting complexity too early.
