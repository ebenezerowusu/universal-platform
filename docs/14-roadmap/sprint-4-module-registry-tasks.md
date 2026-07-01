# Sprint 4 Module Registry Tasks

## Purpose

This document defines the module registry tasks for Sprint 4.

The goal is to prepare module records before domain features are implemented.

## Task 1: Module Catalog Table

Create the first module catalog table.

It should support:

- module key
- module name
- domain key
- version
- status
- description

## Task 2: Organization Module Table

Create the first organization module table.

It should support:

- organization reference
- module reference
- module state
- installation date
- activation date where useful
- configuration metadata where useful

## Task 3: Module Seed Records

Create seed records for:

- core platform modules
- shared engine modules
- first Religious MVP modules

Seed records must use stable module keys.

## Task 4: Module Repository

Create data access methods for:

- listing module catalog records
- listing organization module records
- adding a module to an organization
- updating organization module state
- reading module state

## Task 5: Module Application Services

Create use cases for:

- list available modules
- list organization modules
- add module to organization
- update module state
- check module availability for future use cases

## Completion Standard

The module registry portion of Sprint 4 is complete when module records can be stored, seeded, listed, and checked by application services.

## Final Rule

Module records must be managed through Module Registry. Domains should not invent their own module tracking.
