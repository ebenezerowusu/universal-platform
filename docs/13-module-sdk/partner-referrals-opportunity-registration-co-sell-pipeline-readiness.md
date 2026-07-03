# Partner Referrals Opportunity Registration Co-Sell Pipeline Readiness

## Purpose

This document prepares the Partner Referrals, Opportunity Registration, and Co-Sell Pipeline foundation for implementation.

It ensures referral leads, opportunity registrations, co-sell stages, eligibility checks, duplicate/conflict reviews, revenue/subscription links, customer success/implementation handoffs, partner status updates, and referral audit history are partner-safe, tenant-safe, permission-protected, and auditable.

## Wave Summary

Build foundations for referral lead metadata, opportunity registration metadata, co-sell pipeline stage placeholders, partner eligibility/readiness checks, duplicate/conflict review placeholders, revenue/subscription linkage, customer success/implementation handoff linkage, partner status update placeholders, and referral audit history.

## Problem Being Solved

Partners can help source opportunities and support customer growth, but partner-originated leads need controlled visibility, review, conflict handling, and safe handoff into revenue and delivery workflows.

Without a shared foundation, referrals may be handled through informal channels with unclear ownership and weak audit history.

## Evidence

This wave is supported by:

- partner ecosystem and delivery delegation foundation
- partner enablement and certification foundation
- subscription billing and revenue operations foundation
- customer success foundation
- implementation delivery foundation
- solution blueprints foundation
- support operations foundation
- audit and permission foundations

## Primary Users

- platform admin
- partner/admin user
- partner manager placeholder
- revenue/admin user
- customer success/admin user
- implementation/admin user
- approved partner user with restricted scope

## MVP Scope

Included:

- referral lead metadata
- opportunity registration metadata
- co-sell pipeline stage placeholder
- partner eligibility/readiness check placeholder
- duplicate/conflict review placeholder
- revenue/subscription linkage
- customer success/implementation handoff linkage
- partner status update placeholder
- audit and permission direction

Excluded:

- full CRM replacement
- automatic deal approval
- automatic commission calculation or payout
- legal contract management
- unrestricted partner visibility into tenant/customer records
- bypass of partner certification, tenant approval, or entitlement rules
- AI-only opportunity qualification decisions

## Domain Ownership

Partner Referral Foundation owns referral leads, opportunity registrations, co-sell stages, eligibility snapshots, conflict reviews, partner-safe status updates, and referral audit history.

Partner Ecosystem owns partner organizations, implementer profiles, delegations, assignments, and partner access state.

Partner Enablement owns certification and readiness records.

Subscription Billing and Revenue Operations owns plans, subscriptions, billing, revenue records, and pricing operations.

Customer Success owns account health, customer success handoffs, and adoption playbooks.

Implementation Delivery owns onboarding projects and migration delivery records.

Permission Engine controls referral and partner visibility.

Audit Engine records referral and opportunity actions.

## Required Engines

- Partner Referral Foundation
- Permission Engine
- Audit Engine
- Partner Ecosystem Foundation
- Partner Enablement Foundation
- Subscription Billing/Revenue Operations Foundation
- Customer Success Foundation
- Implementation Delivery Foundation
- Solution Blueprint Foundation
- Notification Engine
- Workflow Engine

## Suggested Permissions

```text
partner_referrals.dashboard.view
partner_referrals.referrals.view
partner_referrals.referrals.submit
partner_referrals.referrals.manage
partner_referrals.opportunities.view
partner_referrals.opportunities.manage
partner_referrals.pipeline.view
partner_referrals.pipeline.manage
partner_referrals.eligibility.view
partner_referrals.conflicts.view
partner_referrals.conflicts.manage
partner_referrals.handoffs.view
partner_referrals.handoffs.manage
partner_referrals.audit_history.view
```

## Data Ownership

Records should include:

- referral lead metadata
- opportunity registration metadata
- co-sell pipeline stage placeholder
- eligibility/readiness snapshot placeholder
- duplicate/conflict review placeholder
- revenue/subscription link
- customer success/implementation handoff link
- partner-safe status update placeholder
- referral audit event

## API Direction

Planned API groups:

- partner referral dashboard
- referral leads
- opportunity registrations
- co-sell pipeline stages
- eligibility/readiness reviews
- duplicate/conflict reviews
- revenue/subscription links
- customer success/implementation handoffs
- partner status updates
- referral audit history

## UI Direction

Planned screens:

- partner referral dashboard
- referral lead list
- opportunity registration list
- opportunity detail
- co-sell pipeline board
- eligibility/readiness review
- duplicate/conflict review
- revenue/subscription linkage view
- handoff view
- referral audit history

## Audit Direction

Audit important actions:

- referral submitted
- referral updated or closed
- opportunity registered or updated
- pipeline stage changed
- eligibility/readiness reviewed placeholder
- duplicate/conflict review recorded placeholder
- revenue/subscription link changed
- customer success/implementation handoff created
- partner status update sent placeholder

## Revenue Direction

This wave supports partner-sourced growth, controlled co-selling, expansion opportunities, partner-led implementation handoff, and future partner program monetization without implementing commission or payout logic now.

## Risks

- partner sees restricted customer details
- multiple partners claim the same prospect without review
- opportunity is treated as committed revenue
- referral bypasses partner readiness checks
- partner receives status updates with sensitive internal details
- commission/payout assumptions are created without finance controls

## Readiness Decision

```text
ready for implementation planning
```

## Final Rule

Partner referrals should turn partner-sourced interest into reviewable opportunities while preserving privacy, partner boundaries, revenue ownership, and audit history.
