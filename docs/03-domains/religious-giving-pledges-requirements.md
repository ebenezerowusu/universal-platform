# Religious Giving And Pledges Requirements

## Purpose

This document defines the requirements for Religious Giving, Church Payments, Pledge Campaigns, and Pledge tracking in the Religious Domain.

The feature must support different church and institution practices. Some organizations will define a campaign target amount. Others will only track promised giving without a target.

## Core Requirement

A pledge campaign may have a target amount, but the target amount must be optional.

```text
target_amount nullable
```

This means:

- an organization may create a campaign with a fixed target amount
- another organization may create a campaign without any target amount
- the system must not force every pledge campaign to have a target
- reports and dashboards must adapt based on whether a target exists

## Pledge Campaign Examples

### Target-Based Campaign

```text
Campaign: Building Project
Target Amount: GH₵500,000
```

Used when the church wants to raise a known amount.

### Non-Target Campaign

```text
Campaign: Harvest Pledge 2026
Target Amount: not set
```

Used when the church only wants to record what people promised and what they paid, without comparing it to a fixed target.

## Pledge Campaign Fields

Suggested fields:

```text
id
organization_id
branch_id nullable
name
description nullable
target_amount nullable
currency
start_date nullable
end_date nullable
status
allow_public_pledges
allow_anonymous_pledges
created_by
updated_by nullable
created_at
updated_at
archived_at nullable
```

## Dashboard Rules

### When `target_amount` Is Set

Show:

```text
Target Amount
Total Pledged
Total Paid
Outstanding Pledges
Target Progress
```

Example:

```text
Target Amount: GH₵500,000
Total Pledged: GH₵320,000
Total Paid: GH₵180,000
Outstanding Pledges: GH₵140,000
Target Progress: 36%
```

Target Progress formula:

```text
Target Progress = Total Paid / Target Amount
```

### When `target_amount` Is Not Set

Show only:

```text
Total Pledged
Total Paid
Outstanding Pledges
```

Example:

```text
Total Pledged: GH₵320,000
Total Paid: GH₵180,000
Outstanding Pledges: GH₵140,000
```

Do not show:

```text
Target: Not set
Progress: Not applicable
```

UI rule:

```text
If target_amount is null, hide Target Amount and Target Progress completely.
```

## Amount Calculations

Required calculations:

```text
Total Pledged = sum of active pledge amounts for the campaign
Total Paid = sum of confirmed pledge payments for the campaign
Outstanding Pledges = Total Pledged - Total Paid
```

Outstanding pledge amount must not be negative in normal reporting.

If overpayment is allowed later, it must be handled explicitly as an overpayment/extra giving case.

## Pledge Statuses

Suggested statuses:

```text
draft
active
partially_paid
completed
overdue
cancelled
written_off
under_review
```

Status meaning:

- `active`: pledge has been recorded and remains open
- `partially_paid`: at least one payment exists but balance remains
- `completed`: paid amount is equal to or greater than pledged amount
- `overdue`: due date has passed and balance remains
- `cancelled`: pledge is no longer expected
- `written_off`: church has approved not collecting the remaining balance
- `under_review`: pledge needs finance/admin review

## Who Can Make A Pledge

Supported pledger types:

```text
member
household
group
branch
anonymous
public_donor
corporate_donor_placeholder
```

The system must not assume that every pledge belongs only to one individual member.

## Pledge Payments

A pledge payment is a payment made toward a pledge.

Payment sources:

```text
member_mobile_app
public_giving_page
admin_manual_entry
payment_provider_callback
imported_record
```

Payment methods:

```text
mobile_money
card
bank_transfer_placeholder
cash
cheque
pos_placeholder
```

Every pledge payment should link to a giving transaction so finance reports remain consistent.

Recommended structure:

```text
Pledge
  -> PledgePayment
    -> GivingTransaction
      -> PaymentTransaction placeholder
      -> Receipt
```

## Receipts

Every confirmed pledge payment should be receiptable.

Receipt should include:

```text
church or organization name
branch
campaign name
pledge reference
payment amount
total pledged
total paid so far
remaining balance
payment method
transaction reference
receipt number
date
```

Receipts may be sent through:

```text
Email
SMS
WhatsApp
in-app notification
printable PDF placeholder
```

## Reminders

Pledge reminders should support:

```text
SMS
Email
WhatsApp
in-app notification
push notification placeholder
```

Reminder rules should be configurable by organization and campaign.

Examples:

```text
send reminder before due date
send reminder on due date
send reminder after overdue
send monthly pledge balance reminder
use WhatsApp first, fallback to SMS
```

Reminder sending must respect communication preferences and paid communication channel balances where applicable.

## Permissions

Suggested permissions:

```text
religious.pledges.view
religious.pledges.manage
religious.pledges.create
religious.pledges.update
religious.pledges.cancel
religious.pledges.adjust
religious.pledges.write_off
religious.pledges.export
religious.pledges.reminders.send
religious.pledge_payments.view
religious.pledge_payments.record
religious.pledge_payments.approve_manual
religious.pledge_payments.export
religious.campaigns.view
religious.campaigns.manage
```

Visibility rules:

- members may view only their own pledges
- branch finance officers may view pledges for their branch scope
- higher-level finance users may view roll-up reports based on organization hierarchy permissions
- anonymous donor identity must be hidden except from authorized finance/admin users

## Security And Audit

Pledges and pledge payments are financial records.

Security rules:

```text
never allow negative pledge amounts
never trust frontend payment status
do not mark pledge paid until transaction is confirmed
prevent duplicate payment callbacks
use idempotency keys for payment confirmation
audit manual pledge payments
audit pledge adjustments
audit pledge write-offs
audit pledge exports
protect anonymous donor identity
validate currency
validate campaign status before accepting pledge
```

Audit events:

```text
audit.religious.pledge.created
audit.religious.pledge.updated
audit.religious.pledge.adjusted
audit.religious.pledge.cancelled
audit.religious.pledge.written_off
audit.religious.pledge.exported
audit.religious.pledge_payment.recorded
audit.religious.pledge_payment.approved
audit.religious.pledge_reminder.sent
```

## Reports

Required reports:

```text
pledge summary report
campaign pledge report
member pledge statement
outstanding pledge report
overdue pledge report
completed pledge report
branch pledge report
group pledge report
anonymous pledge report
pledge payment report
pledge reminder report
pledge write-off report
```

Report filters:

```text
campaign
branch
congregation
group
member
status
date range
amount range
payment method
outstanding balance
overdue only
```

Exports must be permission-controlled and audited.

## MVP Scope

Build first:

```text
create pledge campaign with optional target_amount
create member pledge
record pledge payment
track amount pledged, amount paid, and outstanding balance
show member pledge history
show admin campaign progress only when target_amount exists
show total pledged, total paid, and outstanding pledges when target_amount is null
send manual pledge reminder through SMS/Email/WhatsApp
issue receipt for pledge payment
basic pledge reports
```

Delay until later:

```text
complex installment schedules
automatic recurring deductions
full bank reconciliation automation
public donor pledge portal
advanced write-off workflow
tax receipt rules
accounting journal automation
```

## Final Rule

`target_amount` is optional. If it is not set, the UI and reports must hide Target Amount and Target Progress completely and show only Total Pledged, Total Paid, and Outstanding Pledges.
