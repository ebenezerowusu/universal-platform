# Partner Referrals Opportunity Registration Co-Sell Pipeline Guide

## Purpose

This document defines operating guidance for referral leads, opportunity registrations, co-sell pipeline stages, eligibility/readiness reviews, duplicate/conflict reviews, revenue/subscription links, customer success/implementation handoffs, partner status updates, and referral audit history.

The goal is to make partner-sourced opportunities reviewable, safe, and traceable.

## Principle

Partner referrals should help create growth opportunities without becoming a CRM, commission payout engine, or uncontrolled partner visibility path.

Opportunities are reviewable business records, not committed revenue.

## Data Sources

Partner referral operations may use:

- referral lead metadata
- opportunity registration metadata
- co-sell pipeline stage placeholder
- eligibility/readiness snapshot placeholder
- duplicate/conflict review placeholder
- revenue/subscription link
- customer success/implementation handoff link
- partner-safe status update placeholder
- audit records

## Referral Direction

Referral lead records should capture:

```text
referral_key
partner_organization_id
submitted_by_safe_label
prospect_safe_summary
country_region_placeholder
status
```

## Opportunity Direction

Opportunity registration records should capture:

```text
opportunity_key
source_referral_id
target_organization_prospect_placeholder
product_module_interest_placeholder
estimated_value_placeholder
review_status
```

## Pipeline Direction

Pipeline stages may include:

- submitted
- accepted for review
- qualified placeholder
- co-sell in progress
- converted to subscription opportunity placeholder
- closed placeholder

## Eligibility Direction

Eligibility/readiness checks may include:

- partner active status
- implementer certification status
- assignment/delegation eligibility
- country/region eligibility
- solution blueprint certification placeholder

## Conflict Review Direction

Conflict reviews may include:

- duplicate referral warning
- existing customer/prospect conflict
- multiple partner claim warning
- internal sales ownership conflict placeholder
- partner visibility restriction warning

## Revenue Link Direction

Revenue/subscription links may connect to:

- subscription plan interest
- billing/revenue operation record placeholder
- pricing/package placeholder
- payment readiness placeholder
- renewal/expansion opportunity placeholder

## Handoff Direction

Customer success and implementation handoffs may connect to:

- customer success account setup
- implementation project placeholder
- onboarding delivery project
- solution blueprint recommendation
- support readiness notes

## Partner Status Update Direction

Partner status updates should contain safe summaries only.

They should not expose restricted tenant data, internal pricing strategy, unresolved conflicts, or sensitive customer details.

## Audit Direction

Audit should record:

- referral submitted
- referral updated or closed
- opportunity registered or updated
- pipeline stage changed
- eligibility/readiness reviewed placeholder
- duplicate/conflict review recorded placeholder
- revenue/subscription link changed
- customer success/implementation handoff created
- partner status update sent placeholder

## Data Quality Warnings

Warn or flag when:

- referral has no partner organization
- opportunity has no product/module interest
- duplicate referral is suspected
- multiple partners claim the same prospect
- partner readiness check is missing
- partner status update includes unsafe detail placeholder
- opportunity has revenue link but no review status

## Final Rule

Partner referral operations must preserve partner boundaries, tenant privacy, revenue ownership, conflict review, and audit history.
