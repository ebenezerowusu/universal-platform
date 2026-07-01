# Expense Foundation Flutter Contract

## Purpose

This document defines the MVP Flutter screen contract for organization expense foundation.

The screens should help organization users request, approve, record, and review expenses without exposing unnecessary finance complexity.

## Navigation Direction

Expense screens should appear under Finance when:

- user is authenticated
- organization is selected
- Finance module is available
- user has required permission
- feature availability allows the section

## Expense Dashboard

Should show:

- recent expense requests
- recent expense records
- pending approvals where permitted
- budget summary where available
- quick action to create expense request

## Expense Categories Screen

Should show:

- category name
- code or safe label
- status
- edit action where permitted

## Budget Lines Screen

Should show:

- category
- period
- amount
- currency
- status
- notes preview

## Expense Request Form

Should support:

- category selection
- budget line selection optional
- amount
- currency
- purpose
- required date optional
- attachment/reference placeholder where supported
- submit action

## Approval Queue

Should show:

- requester
- category
- amount
- currency
- purpose preview
- submitted date
- approve/reject actions where permitted

## Expense History Screen

Should show:

- category
- amount
- currency
- status
- spent date
- related request where available

## Expense Detail Screen

Should show:

- category
- budget line where available
- request status
- approval history where available
- expense record details
- attachment/reference where available
- audit-friendly timestamps where available

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 21:

- full accounting reports
- payroll screens
- tax screens
- full procurement screens
- vendor management screens
- payment provider setup screens

## Final Rule

Flutter should make expense requests and approvals clear while keeping the finance foundation simple and auditable.
