# Fourth Sprint Plan

## Purpose

This document defines the recommended fourth sprint after Permission and Audit foundations.

The fourth sprint should implement Module Registry, Subscription foundation, and engine skeletons.

## Sprint Goal

Prepare the platform to enable modules and shared capability engines before product domain implementation.

## Prerequisites

Before this sprint begins:

- Backend foundation exists.
- Identity foundation works.
- Organization foundation works.
- Permission foundation works.
- Audit foundation works.

## Sprint Scope

### 1. Module Registry Tables

Create migrations for:

- modules
- organization modules
- module dependencies where practical
- module settings foundation where practical

Expected result:

- Modules can be registered and assigned to organizations.

### 2. Module Registry Services

Implement:

- list modules
- organization module list
- install module
- activate module
- deactivate module
- module availability check

### 3. Subscription Tables

Create migrations for:

- subscription plans
- plan features
- plan limits
- organization subscriptions

Expected result:

- Plan and feature access data can be stored.

### 4. Subscription Services

Implement:

- current organization subscription lookup
- feature access check
- limit check foundation

### 5. Engine Skeletons

Create interfaces and local/mock implementations for:

- Payment Engine
- SMS Engine
- Storage Engine
- Notification Engine
- Reporting Engine
- Workflow Engine

### 6. Seed Data

Seed:

- initial modules
- initial permissions if not complete
- starter subscription plans
- required engine module records

### 7. Tests

Create tests for:

- install module
- activate module
- inactive module blocks feature
- subscription feature check
- engine interface can be called through application service

## Out of Scope

Do not implement:

- live payment provider
- live SMS provider
- advanced billing automation
- full Religious Domain features
- Flutter UI

## Completion Standard

The sprint is complete when:

- Modules can be registered and activated.
- Organization module availability can be checked.
- Subscription feature check exists.
- Engine interfaces exist.
- Mock/local engine implementations exist.
- Tests cover module and feature access checks.

## Final Rule

Sprint 4 prepares modules and engines. Product domain work comes after module and engine foundations are usable.
