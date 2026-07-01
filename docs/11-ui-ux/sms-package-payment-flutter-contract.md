# SMS Package Payment Flutter Contract

## Purpose

This document defines the MVP Flutter screen contract for SMS package payment readiness.

The screens should help organization users select an SMS package, start payment, check status, and understand wallet crediting without exposing payment provider implementation details.

## Navigation Direction

SMS package purchase screens should appear under Communication / SMS Wallet when:

- user is authenticated
- organization is selected
- Communication module is available
- SMS wallet feature is available
- user has required permission

## Package Catalog Screen

Should show:

- package name
- SMS units
- price
- currency
- package status
- purchase action where permitted

## Purchase Confirmation Screen

Should show:

- selected package
- units
- amount
- currency
- payment method hint where supported
- confirm purchase action

Do not show low-level payment provider details.

## Payment Status Screen

Should show:

- purchase status
- payment status
- wallet credit status
- amount
- units
- next action or support guidance

Suggested user-facing states:

- waiting for payment
- processing payment
- payment confirmed
- wallet credited
- payment failed
- needs support review

## Wallet Transaction History

Should show:

- transaction type
- units
- related purchase reference where available
- created date
- status

## Error and Empty States

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 19:

- provider credential screens
- low-level payment provider screens
- full invoice screens
- subscription plan management
- advanced finance dashboards

## Final Rule

Flutter should guide the organization through package purchase and wallet credit status without exposing payment-provider complexity.
