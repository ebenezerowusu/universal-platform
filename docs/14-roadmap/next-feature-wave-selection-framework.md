# Next Feature Wave Selection Framework

## Purpose

This document defines how Universal Platform should choose the next feature wave after launch.

The goal is to prevent random expansion and select work that is valuable, feasible, and aligned with the platform vision.

## Principle

The next feature wave should be selected by evidence, not excitement.

## Evaluation Criteria

Score each candidate from 1 to 5 for:

```text
user_need
revenue_potential
strategic_alignment
architecture_readiness
implementation_size
support_impact
risk_level
reuse_potential
```

## User Need

Ask:

- Did users request this repeatedly?
- Does it solve a real workflow problem?
- Does it reduce support confusion?
- Does it improve retention or repeated usage?

## Revenue Potential

Ask:

- Can this support subscription upgrades?
- Can this support add-on revenue?
- Can this support usage-based revenue?
- Can this make the platform more valuable to organizations?

## Strategic Alignment

Ask:

- Does this fit the platform vision?
- Does it strengthen reusable engines?
- Does it support multiple future domains?
- Does it avoid locking the platform into one narrow market?

## Architecture Readiness

Ask:

- Are required engines ready?
- Are permissions defined?
- Is module availability defined?
- Are data ownership and tenant boundaries clear?
- Are API and Flutter foundations ready enough?

## Implementation Size

Ask:

- Can it fit in one focused wave?
- Can it be delivered incrementally?
- Can MVP scope be clearly limited?
- Does it require large refactors first?

## Support Impact

Ask:

- Will this reduce support burden?
- Will this create training needs?
- Will this introduce operational complexity?
- Can support explain it clearly?

## Risk Level

Ask:

- Does it affect sensitive data?
- Does it affect money movement or external providers?
- Does it create reliability risks?
- Does it create user trust risks?

## Reuse Potential

Ask:

- Can this become a capability engine?
- Can multiple domains use it later?
- Can it improve platform extensibility?

## Decision Rule

Prefer feature waves that:

- have clear user need
- improve revenue path
- fit platform architecture
- can be delivered in a limited scope
- strengthen reusable capabilities
- do not create unmanaged support risk

## Final Rule

Choose one primary wave. Do not approve several large waves at once.
