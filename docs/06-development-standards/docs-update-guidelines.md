# Docs Update Guidelines

## Purpose

This document explains how Universal Platform documentation should stay current after coding begins.

## Principle

Docs should move with the product.

When a change affects behavior, the related document should be reviewed.

## When to Review Docs

Review docs when changing:

- API routes
- database tables
- module behavior
- engine behavior
- domain behavior
- configuration behavior
- deployment behavior
- Flutter navigation
- MVP scope

## PR Guidance

Each pull request should answer:

```text
Does this change affect documentation?
```

If yes, update the related docs.

If no, mention that no documentation update was needed.

## Main Ownership Direction

Use the architecture area as the owner:

- Core changes should review Core docs.
- Engine changes should review Engine docs.
- Domain changes should review Domain docs.
- API changes should review API docs.
- Database changes should review Database docs.
- Flutter changes should review Flutter docs.
- Infrastructure changes should review Infrastructure docs.

## Decision Records

Use ADRs when a change affects major architecture direction.

Examples:

- backend structure
- database strategy
- deployment strategy
- module boundaries
- provider approach
- API versioning

## Quality Checklist

A useful document should be:

- clear
- current
- consistent with platform terms
- specific enough to guide implementation
- easy to find from the documentation index

## Final Rule

Do not let docs become old notes. Keep them aligned with the current platform direction.
