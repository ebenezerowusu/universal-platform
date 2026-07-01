# Sprint 15 Wave Review

## Purpose

This document defines review checks for Sprint 15.

Sprint 15 prepares the next feature wave after launch and stabilization.

## Review 1: Scope

Sprint 15 may include:

- evidence-backed feature wave planning
- module-readiness notes
- feature selection records
- revenue direction
- architecture guardrail updates where needed
- small enabling fixes before the next feature wave

Sprint 15 should not include:

- a new module without feature-wave approval
- random feature requests without evidence
- domain logic inside Platform Core
- provider-specific logic outside adapters
- broad refactors unrelated to the selected wave

## Review 2: Evidence

Confirm evidence was reviewed from:

- adoption metrics
- support themes
- post-launch review
- repeated user requests
- workflow drop-offs
- revenue opportunities

## Review 3: Candidate Waves

Confirm candidate waves were compared, such as:

- Religious v2 improvements
- communication and SMS wallet foundation
- giving and basic finance foundation
- Commerce or POS foundation
- reporting improvements
- import/export improvements
- subscription and billing improvements

## Review 4: Selection Criteria

Confirm candidates were evaluated for:

- user need
- revenue potential
- strategic alignment
- architecture readiness
- implementation size
- support impact
- risk level
- reuse potential

## Review 5: Readiness

Confirm the selected wave has clear:

- owner
- MVP scope
- required engines
- module direction
- data ownership
- API direction
- Flutter direction
- permissions
- audit behavior

## Review 6: Revenue Direction

Confirm:

- revenue path is clear or deferred intentionally
- feature availability can be controlled
- usage can be measured where needed
- support can explain the value

## Review 7: Architecture Safety

Confirm:

- Core remains domain-agnostic
- engines are used correctly
- module boundaries remain clear
- organization ownership rules are clear
- external providers remain behind adapters

## Review 8: Decision

Choose one:

```text
select next feature wave
select next feature wave with discovery items
continue stabilization
pause expansion
```

## Final Rule

Sprint 15 is complete when the next feature wave is selected or deferred with evidence, readiness, risk, and revenue direction documented.
