# Religious Giving and Pledges Specification

## Purpose

This document defines giving, donations, and pledge tracking inside the Religious Domain.

The Religious Domain owns giving meaning and categories. The Payment Engine owns payment processing.

## Scope

Giving and pledge features may include:

- Giving categories
- Member giving records
- Anonymous giving records
- Donation records
- Pledge commitments
- Pledge payment tracking
- Receipts
- Giving history
- Branch giving reports
- Project/fund support

## Giving Types

Possible giving types:

- tithe
- offering
- donation
- welfare
- project
- event
- pledge_payment

Labels should be configurable by organization.

## Relationship to Payment Engine

The Religious Domain must not call payment providers directly.

Correct flow:

```text
Religious Giving
  -> Payment Engine
    -> Provider Resolver
      -> Provider Adapter
```

## Pledge Flow

```text
Member creates pledge or admin records pledge
  -> Pledge schedule tracked
  -> Reminder may be sent
  -> Payment recorded against pledge
  -> Pledge progress updated
```

## Suggested Entities

Candidate entities:

- religious_giving_categories
- religious_giving_records
- religious_pledges
- religious_pledge_payments
- religious_giving_receipts

## Required Platform Engines

This module must use:

- Organization Engine for tenant context
- Permission Engine for access control
- Payment Engine for digital payments
- Notification Engine for receipts/reminders
- SMS Engine through Notification Engine where applicable
- Reporting Engine for giving reports
- Audit Engine for sensitive financial changes

## Permission Examples

- religious.giving.view
- religious.giving.record
- religious.giving.manage_categories
- religious.giving.view_reports
- religious.pledges.view
- religious.pledges.manage

## Events Published

- ReligiousGivingRecorded
- ReligiousPledgeCreated
- ReligiousPledgePaymentRecorded
- ReligiousGivingReceiptGenerated

## Events Consumed

- PaymentCompleted
- PaymentFailed
- NotificationDelivered

## Reports

Reports may include:

- Giving by category
- Giving by branch
- Giving by date range
- Pledge progress
- Project support progress
- Member giving summary where permitted

## Security Requirements

- Giving records are sensitive.
- Financial reports require explicit permission.
- Exports should be audited.
- Payment status must come from Payment Engine verification.
- Client-provided payment status must not be trusted.

## Testing Requirements

Tests must cover:

- Create giving category
- Record giving
- Create pledge
- Record pledge payment
- PaymentCompleted event handling
- Permission checks
- Organization boundary checks
- Report access checks

## Final Rule

Religious Giving owns giving meaning. Payment Engine owns payment processing and verification.
