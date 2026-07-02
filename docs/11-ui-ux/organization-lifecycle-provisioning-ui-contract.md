# Organization Lifecycle Provisioning UI Contract

## Purpose

This document defines the MVP UI contract for Organization Lifecycle, Provisioning, and Tenant Operations foundation.

The screens should help permitted users review provisioning requests, lifecycle status, activation/suspension actions, admin bootstrap, regional bootstrap, module bootstrap, billing linkage, onboarding linkage, and tenant isolation validation placeholders.

## Navigation Direction

Organization lifecycle screens should appear when:

- user is authenticated
- organization is selected
- lifecycle feature is available
- user has required permission
- feature availability allows lifecycle operations

## Lifecycle Dashboard

Should show:

- organization lifecycle status
- provisioning status
- admin bootstrap status
- regional bootstrap status
- module bootstrap status
- billing linkage status
- tenant validation status
- onboarding status where available

## Provisioning Request List

Should show:

- organization name
- requested country
- requested module count
- status
- requested by safe label
- created date
- action to view detail

## Provisioning Request Detail

Should show:

- organization name
- requested country
- requested modules
- owner/admin user safe label
- status
- source channel placeholder
- notes where permitted
- approve/reject placeholder actions where permitted

## Lifecycle Action Panel

Should support:

- activate placeholder
- suspend placeholder
- reactivate placeholder
- archive placeholder
- action reason
- confirmation prompt

Destructive organization deletion should not appear in MVP.

## Admin Bootstrap Summary

Should show:

- owner/admin user safe label
- role assignment placeholder
- invite status placeholder
- bootstrap status
- last updated date

## Regional Bootstrap Summary

Should show:

- country
- currency
- timezone
- default language
- supported languages
- format summary
- validation warnings where available

## Module Bootstrap Summary

Should show:

- requested modules
- installed modules
- entitlement status
- module bootstrap status
- warnings where available

## Billing Linkage Summary

Should show:

- subscription reference safe label
- plan key
- billing status
- entitlement status
- warning count where available

## Tenant Isolation Validation Summary

Should show:

- validation status
- last run date
- passed checks
- warning checks
- failed checks
- safe messages

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 43:

- destructive organization deletion UI
- hidden tenant data browser UI
- direct payment provider setup UI
- provider-specific infrastructure provisioning UI
- country-specific provisioning UI inside Core

## Final Rule

Organization lifecycle UI should make tenant readiness visible without giving operators unsafe access to tenant data.
