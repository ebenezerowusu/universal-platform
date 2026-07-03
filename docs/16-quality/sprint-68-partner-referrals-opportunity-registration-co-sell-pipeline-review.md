# Sprint 68 Partner Referrals Opportunity Registration Co-Sell Pipeline Review

## Purpose

This document defines review checks for Sprint 68.

Sprint 68 implements Partner Referrals, Opportunity Registration, and Co-Sell Pipeline foundation.

## Review 1: Scope

Sprint 68 may include:

- partner referral feature registration foundation
- referral lead metadata foundation
- opportunity registration metadata foundation
- co-sell pipeline stage placeholder foundation
- partner eligibility/readiness check placeholder where practical
- duplicate/conflict review placeholder where practical
- revenue/subscription linkage where practical
- customer success/implementation handoff linkage where practical
- partner status update placeholder where practical
- permission and feature checks
- audit records for referral, opportunity, and co-sell actions
- API and UI foundations

Sprint 68 should not include:

- full CRM replacement
- automatic deal approval
- automatic commission calculation or payout
- legal contract management
- unrestricted partner visibility into tenant/customer records
- bypass of partner certification, tenant approval, or entitlement rules
- AI-only opportunity qualification decisions
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Partner Referral Foundation owns referral leads, opportunity registrations, co-sell stages, eligibility snapshots, conflict reviews, partner-safe status updates, and referral audit history
- Partner Ecosystem owns partner organizations, implementer profiles, delegations, assignments, and partner access state
- Partner Enablement owns certification and readiness records
- Subscription Billing and Revenue Operations owns plans, subscriptions, billing, revenue records, and pricing operations
- Customer Success owns account health, customer success handoffs, and adoption playbooks
- Implementation Delivery owns onboarding projects and migration delivery records
- Permission Engine controls referral and partner visibility
- Audit Engine records referral and opportunity actions

## Review 3: Partner and Visibility Rules

Confirm:

- partners only see records they are permitted to see
- partner status updates are safe summaries only
- referral records do not expose restricted tenant or prospect data
- partner readiness checks do not automatically approve opportunities
- partner certification does not bypass entitlement or tenant approval rules

## Review 4: Revenue and Opportunity Rules

Confirm:

- opportunities do not imply committed revenue
- revenue/subscription links do not mutate billing records directly
- conflict review is available where duplicate/multiple claims exist
- co-sell stage changes are audited
- commission/payout logic is not implemented

## Review 5: Warning Rules

Confirm warnings exist where practical for:

- referral has no partner organization
- opportunity has no product/module interest
- duplicate referral is suspected
- multiple partners claim the same prospect
- partner readiness check is missing
- partner status update includes unsafe detail placeholder
- opportunity has revenue link but no review status

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/partner-referrals-opportunity-registration-co-sell-pipeline-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/partner-referrals-opportunity-registration-co-sell-pipeline-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- referral lead creation/update/close placeholder
- opportunity creation/update/review placeholder
- pipeline stage creation/update
- eligibility review creation/review placeholder
- conflict review creation/resolve placeholder
- revenue/subscription link creation/update
- customer success/implementation handoff creation/update
- partner status update creation/send placeholder
- organization boundary enforcement
- permission denied behavior
- audit record creation

## Final Rule

Sprint 68 is complete when referral leads, opportunities, co-sell stages, eligibility reviews, conflict reviews, revenue/subscription links, customer success/implementation handoffs, partner status updates, and referral audit history are available through partner-safe, tenant-safe, revenue-boundary-aware, permission-protected, review-based, and auditable foundations.
