# Partner Referrals Opportunity Registration Co-Sell Pipeline UI Contract

## Purpose

This document defines the MVP UI contract for Partner Referrals, Opportunity Registration, and Co-Sell Pipeline foundation.

The screens should help permitted users review referral leads, opportunity registrations, co-sell pipeline stages, eligibility/readiness reviews, duplicate/conflict reviews, revenue/subscription links, customer success/implementation handoffs, partner status updates, and referral audit history.

## Navigation Direction

Partner referral screens should appear when:

- user is authenticated
- organization is selected where required
- partner referral feature is available
- user has required permission
- referral visibility rules allow access

## Partner Referral Dashboard

Should show:

- open referral count
- registered opportunity count
- open conflict review count
- active co-sell item count
- pending handoff count
- recent referral audit events

## Referral Lead List

Should show:

- referral safe label
- partner organization safe label
- submitted by safe label
- prospect safe summary
- country/region placeholder
- status
- action to view detail

## Opportunity Registration List

Should show:

- opportunity safe label
- source referral safe label
- product/module interest placeholder
- estimated value placeholder
- review status
- co-sell stage placeholder

## Opportunity Detail

Should show:

- opportunity safe summary
- partner safe summary
- prospect/customer safe summary
- product/module interest placeholder
- eligibility/readiness review summary
- duplicate/conflict review summary
- revenue/subscription links
- customer success/implementation handoff links

## Co-Sell Pipeline Board

Should show stages for:

- submitted
- accepted for review
- qualified placeholder
- co-sell in progress
- converted to subscription opportunity placeholder
- closed placeholder

## Eligibility and Readiness Review

Should show:

- partner active status
- implementer certification status
- assignment/delegation eligibility
- country/region eligibility
- solution blueprint certification placeholder
- review status

## Duplicate and Conflict Review

Should show:

- duplicate referral warning
- existing customer/prospect conflict
- multiple partner claim warning
- internal sales ownership conflict placeholder
- partner visibility restriction warning
- resolution status

## Revenue and Subscription Linkage View

Should show:

- subscription plan interest
- billing/revenue operation placeholder
- pricing/package placeholder
- payment readiness placeholder
- renewal/expansion opportunity placeholder

## Customer Success and Implementation Handoff View

Should show:

- customer success account setup link
- implementation project placeholder
- onboarding delivery project
- solution blueprint recommendation
- support readiness notes
- handoff status

## Partner Status Updates

Should show:

- partner-safe status summary
- target opportunity safe label
- update status
- sent date placeholder
- sent by safe label

## Referral Audit History

Should show:

- event type
- actor safe label
- target safe label
- timestamp
- safe metadata summary

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied
- pending review
- conflict review open

## MVP Exclusions

Do not implement in Sprint 68:

- full CRM UI
- automatic deal approval UI
- commission payout UI
- legal contract management UI
- unrestricted customer/tenant visibility UI
- AI-only opportunity qualification UI

## Final Rule

Partner referral UI should support co-selling visibility and safe handoffs without exposing restricted customer details or implying committed revenue.
