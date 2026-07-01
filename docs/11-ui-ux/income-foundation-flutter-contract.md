# Income Foundation Flutter Contract

## Purpose

This document defines the MVP Flutter screen contract for organization income foundation.

The screens should help organization users manage income categories, commitments, income records, payment status, and receipts without exposing payment provider details.

## Navigation Direction

Income screens should appear under Finance when:

- user is authenticated
- organization is selected
- Finance module is available
- user has required permission
- feature availability allows the section

Religious giving workflows may link into these screens with domain-specific labels where appropriate.

## Income Dashboard

Should show:

- total received for selected period where available
- recent income records
- open commitments where available
- quick action to record income
- quick link to receipts

## Categories Screen

Should show:

- category name
- code or safe label
- status
- edit action where permitted

## Commitment Screen

Should show:

- commitment amount
- currency
- category
- status
- due date where available
- fulfilled amount where available

## Record Income Screen

Should support:

- category selection
- amount
- currency
- received date
- payer reference optional
- anonymous indicator where allowed
- notes
- payment-linked option where supported

## Payment Status Screen

Should show:

- income record status
- payment status where linked
- amount
- currency
- next action or support guidance

Do not expose low-level payment provider details.

## Receipt Screen

Should show:

- receipt number
- amount
- currency
- category
- issued date
- payer label or anonymous label

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 20:

- full accounting reports
- expense management screens
- payroll screens
- tax screens
- full invoicing screens
- payment provider setup screens

## Final Rule

Flutter should make basic income tracking clear and safe while keeping payment provider complexity hidden behind platform APIs.
