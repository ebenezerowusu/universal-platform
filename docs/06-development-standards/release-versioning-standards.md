# Release and Versioning Standards

## Purpose

This document defines release and versioning standards for Universal Platform.

The goal is to make releases predictable as the backend, Flutter clients, modules, and APIs grow.

## Principle

Versioning should communicate change clearly.

A release should explain what changed, what needs attention, and whether any migration or configuration update is required.

## Versioning Areas

Universal Platform may have versions for:

- backend application
- API contracts
- Flutter mobile app
- Flutter desktop/admin app
- modules
- migrations
- provider adapters

## Backend Release Notes

Backend release notes should include:

- features added
- fixes included
- migrations included
- configuration changes
- module changes
- API contract changes
- operational notes

## API Versioning

Initial API base:

```text
/api/v1
```

Do not create new API versions casually.

Prefer backward-compatible changes where practical.

## Module Versioning

Each module should have a version in its manifest or module record.

Module version changes should be documented when behavior, permissions, events, or settings change.

## Migration Versioning

Migration files should be ordered and named clearly.

Migration changes should be included in release notes where production data is affected.

## Flutter Versioning

Flutter releases should track:

- mobile app version
- desktop/admin version where applicable
- backend API compatibility
- required minimum backend version where applicable

## Breaking Change Direction

A breaking change should document:

- what changed
- why it changed
- affected APIs or modules
- migration path
- rollout plan

## Release Checklist

Before release, confirm:

- tests passed
- migrations reviewed
- configuration reviewed
- release notes prepared
- rollback direction considered
- production readiness checks reviewed where applicable

## Final Rule

A release is not just a deployment. It is a documented change to the platform.
