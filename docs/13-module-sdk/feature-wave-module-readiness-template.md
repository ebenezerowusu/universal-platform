# Feature Wave Module Readiness Template

## Purpose

This template should be copied when preparing a new module or major feature wave.

It ensures the feature wave is ready before implementation starts.

## Template

```text
# Feature Wave Readiness: <Wave Name>

## Wave Summary

Short description of the feature wave.

## Problem Being Solved

What user or business problem does this solve?

## Evidence

- adoption metric evidence
- support evidence
- user feedback
- revenue evidence
- strategic reason

## Primary Users

- user type 1
- user type 2

## MVP Scope

Included:

- item 1

Excluded:

- item 1

## Domain Ownership

Core / Religious / Commerce / Finance / HR / Platform Engine / Other

## Required Engines

- Permission Engine
- Audit Engine
- Configuration Engine
- Notification Engine
- Payment Engine where applicable
- SMS Engine where applicable
- Reporting Engine where applicable

## Module Manifest Direction

Required:

- module id
- permissions
- navigation entries
- feature flags
- events published
- events consumed
- migrations
- jobs

## Data Ownership

List key records and organization ownership rules.

## API Direction

List planned API groups.

## Flutter Direction

List planned screens or workflows.

## Permission Direction

List required permission keys.

## Audit Direction

List important changes that should be auditable.

## Revenue Direction

Choose:

- included in base plan
- premium module
- usage-based add-on
- service revenue support
- future revenue only

## Risks

- risk 1

## Readiness Decision

Choose one:

- ready for implementation planning
- needs more discovery
- defer
```

## Final Rule

A feature wave should not begin implementation until ownership, scope, dependencies, data rules, permissions, audit, API, UI, and revenue direction are clear.
