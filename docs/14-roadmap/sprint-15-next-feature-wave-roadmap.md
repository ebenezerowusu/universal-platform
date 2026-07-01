# Sprint 15 Next Feature Wave Roadmap

## Purpose

This document defines Sprint 15 for selecting and preparing the next MVP feature wave.

Sprint 15 should use post-launch evidence to decide what to build next.

## Sprint Goal

Select the next focused feature wave that improves product value without violating platform architecture or expanding randomly.

## Prerequisites

Before Sprint 15 begins:

- post-launch review is complete
- adoption metrics have been reviewed
- support themes have been grouped
- urgent stabilization issues are addressed or accepted
- platform extension points are stable enough for another feature wave

## Work Package 1: Evidence Review

Review:

- adoption metrics
- support themes
- repeated user requests
- workflow drop-offs
- accepted limitations
- revenue opportunities
- architecture readiness

## Work Package 2: Candidate Feature Waves

Possible next waves include:

- Religious v2 improvements
- Communication and SMS wallet foundation
- Giving and basic finance foundation
- Commerce/POS foundation
- Reporting improvements
- Import/export improvements
- Subscription and billing improvements

## Work Package 3: Selection

Use:

```text
docs/14-roadmap/next-feature-wave-selection-framework.md
```

Select one primary wave and optionally one small enabling wave.

## Work Package 4: Module Readiness

For any selected module or feature wave, prepare:

```text
docs/13-module-sdk/feature-wave-module-readiness-template.md
```

## Work Package 5: Revenue Readiness

Review:

```text
docs/14-roadmap/revenue-readiness-roadmap.md
```

Confirm whether the selected wave supports subscriptions, usage, add-ons, or service revenue.

## Work Package 6: Architecture Review

Confirm:

- Core remains domain-agnostic
- engines are used correctly
- module boundaries are clear
- data ownership is clear
- permission and feature checks are planned
- audit behavior is planned

## Decision Options

Choose one:

```text
select Religious v2 wave
select communication/SMS wave
select giving/basic finance wave
select Commerce/POS foundation wave
select reporting/import wave
continue stabilization before new wave
```

## Out of Scope

Do not start implementation for multiple large waves at once.

Do not add features only because they are interesting.

## Completion Standard

Sprint 15 is complete when the next feature wave is selected or explicitly deferred, with readiness, risk, and revenue considerations documented.

## Final Rule

The next feature wave should be chosen by evidence, strategic value, revenue potential, and platform readiness.
