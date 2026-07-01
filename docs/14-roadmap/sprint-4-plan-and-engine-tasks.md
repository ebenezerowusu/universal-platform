# Sprint 4 Plan and Engine Tasks

## Purpose

This document defines the plan access and shared engine tasks for Sprint 4.

It complements the Sprint 4 Module Registry tasks.

## Task 1: Plan Tables

Create the first tables for:

- plans
- plan features
- plan limits
- organization plan assignment

## Task 2: Plan Seed Records

Create starter plan records for MVP testing.

Plan records should use stable plan codes.

## Task 3: Plan Access Services

Create services for:

- current organization plan lookup
- feature availability check
- limit check foundation

## Task 4: Engine Interfaces

Create initial interfaces for shared engines:

- Payment Engine
- SMS Engine
- Storage Engine
- Notification Engine
- Reporting Engine
- Workflow Engine

## Task 5: Local Engine Implementations

Create local or mock implementations where useful.

The goal is to let application services call engines without live provider setup.

## Task 6: Tests

Add tests for:

- plan seed records
- feature availability check
- limit check foundation
- engine interface call

## Completion Standard

Sprint 4 plan and engine work is complete when application services can check plan-based feature availability and call shared engine interfaces without live providers.

## Final Rule

Sprint 4 should prepare shared foundations. It should not implement live provider integrations or full domain workflows.
