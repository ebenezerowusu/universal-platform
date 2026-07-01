# Data Classification and Privacy Guide

## Purpose

This document defines how Universal Platform should classify and protect data.

The platform will handle organizational, personal, financial, communication, care, and operational records. Data handling rules must be clear before implementation grows.

## Classification Principle

Not all data has the same sensitivity.

The platform should classify data and apply controls based on sensitivity, access needs, and business risk.

## Data Classes

### Public Data

Data intended for public viewing.

Examples:

- Public organization name
- Public event title
- Public marketplace listing
- Public branch contact page later

### Internal Data

Data visible to authorized organization users.

Examples:

- Group list
- Service schedule
- General attendance summary
- Non-sensitive reports

### Personal Data

Data that identifies or relates to a person.

Examples:

- Name
- Phone
- Email
- Date of birth
- Address
- Member profile
- Visitor profile

### Sensitive Personal Data

Personal data that requires stricter access.

Examples:

- Confidential care notes
- Counseling records
- Welfare support information
- Family relationship details
- Emergency contact details

### Financial Data

Data related to payments, giving, subscriptions, wallets, and reports.

Examples:

- Giving records
- Payment transaction references
- SMS wallet purchases
- Subscription records
- Financial reports

### Operational Data

Data used for platform operation.

Examples:

- Logs
- Job records
- Provider status
- Audit events
- Trace identifiers

## Protection Rules

### Public Data

- Can be shown publicly if explicitly marked public.
- Must not accidentally expose private fields.

### Internal Data

- Requires authentication.
- Requires organization access.
- May require module and permission checks.

### Personal Data

- Requires organization access.
- Requires appropriate permission.
- Exports should be controlled.

### Sensitive Personal Data

- Requires explicit permission.
- Access should be audited where appropriate.
- Should not appear in general lists by default.
- Should not be included in broad exports without clear permission.

### Financial Data

- Requires explicit permission.
- Important changes should be audited.
- Reports and exports should be controlled.
- Client-provided payment status must not be trusted.

### Operational Data

- Logs should avoid sensitive content.
- Audit logs should not store raw secrets.
- Trace identifiers should help investigation without exposing private data.

## Privacy by Design

Implementation should follow these rules:

- Collect only required data.
- Show only data needed for the user role.
- Keep organization data separated.
- Audit sensitive access where needed.
- Do not expose sensitive details through error messages.
- Do not store secrets in regular settings tables.

## Testing Requirements

Tests should cover:

- Sensitive route permission checks
- Organization boundary enforcement
- Export permission checks
- Confidential record access checks
- Safe error responses

## Final Rule

Data access must be intentional. Sensitive data should never be exposed by accident through lists, reports, logs, exports, or weak permissions.
