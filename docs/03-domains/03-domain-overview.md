# Domain Overview

## Purpose

Domains contain business-specific logic.

The Platform Core provides shared capabilities. Domains use those capabilities to implement business workflows for specific industries or operational areas.

## Domain Rule

A domain may use Platform Core capabilities and Capability Engines, but the Platform Core must not depend on any domain.

```text
Platform Core -> no dependency on domains
Domains -> depend on platform contracts and engines
```

## Initial Domains

### Religious Domain

The first major domain.

It covers religious organization management, including:

- Members
- Visitors
- Branch hierarchy
- Congregations
- Services
- Groups
- Bible Study programs
- Attendance
- Giving
- Prayer and counseling
- Welfare
- Events
- Communication
- Reports

### Commerce Domain

Future domain for marketplace and trade activity.

It may include:

- Shops
- Product catalog
- Marketplace listings
- Cart
- Checkout
- Orders
- Delivery integration
- Promotions
- Reviews

### POS Domain / Module

Business-facing module for shops and marts.

It may include:

- Inventory
- Products
- Sales
- Receipts
- Cashiers
- Stock movement
- Barcode support
- Daily sales reports

### Finance Domain

Financial management module.

It may include:

- Chart of accounts
- Income
- Expenses
- Budgets
- Funds
- Approvals
- Financial reports
- Reconciliation

### HR Domain

Human Resource Management for staff/employees.

It may include:

- Staff profiles
- Departments
- Attendance
- Leave management
- Payroll foundation
- Documents
- Performance tracking

### Logistics Domain

Future domain for delivery, rider, and transport workflows.

It may include:

- Delivery requests
- Riders/drivers
- Dispatch
- Tracking
- Delivery fees
- Trip lifecycle

## Domain Design Requirements

Every domain document must define:

- Purpose
- Scope
- What is included
- What is excluded
- Entities
- Workflows
- Permissions
- Events published
- Events consumed
- Reports
- Configuration
- API requirements
- Data ownership
- Integration with capability engines

## Domain Isolation

Domains must not directly access another domain's private tables.

Domains may communicate through:

- Events
- Public application services
- Shared capability engines
- Stable interfaces

## Example

The Religious Domain should not call Arkessel directly for birthday messages.

Correct flow:

```text
Religious Domain
  -> SMS Engine
    -> SMS Provider Resolver
      -> Arkessel Adapter
```

The Commerce Domain should not call Hubtel directly for checkout.

Correct flow:

```text
Commerce Domain
  -> Payment Engine
    -> Payment Provider Resolver
      -> Hubtel Adapter
```

## First Domain to Specify

The first detailed domain specification should be:

```text
docs/03-domains/religious-domain.md
```

That document should be completed before the first Religious Domain code is written.
