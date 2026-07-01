# Migration Implementation Checklist

## Purpose

This checklist helps developers and coding agents create safe database migrations.

It complements the database strategy, migration standards, and data model documents.

## Before Creating a Migration

Confirm:

- Which feature requires the migration
- Which engine or domain owns the table
- Whether the data is platform-level or organization-owned
- Whether existing data is affected
- Whether seed data is needed

## Table Checklist

For each new table, verify:

- table name follows project naming direction
- primary key exists
- timestamps exist
- organization context exists when needed
- status field exists where lifecycle matters
- foreign keys are considered
- indexes are considered
- soft delete is considered where appropriate

## Organization-Owned Table Checklist

For organization-owned tables, verify:

- organization context is included
- common queries include organization context
- uniqueness is scoped by organization where needed
- reports can filter by organization context

## Index Checklist

Consider indexes for:

- organization context
- status
- created date
- updated date
- foreign keys
- report date fields
- common filters

Do not add indexes without an expected query need.

## Constraint Checklist

Consider constraints for:

- required fields
- foreign keys
- unique rules
- duplicate prevention
- valid status values where practical

## Data Change Checklist

If the migration changes existing data, verify:

- impact is understood
- backfill is planned
- rollback or recovery is considered
- deployment order is safe

## Review Checklist

Before merge, verify:

- migration applies cleanly
- migration name is clear
- table ownership is clear
- indexes are appropriate
- no domain concept leaks into Platform Core
- docs are updated where needed

## Final Rule

A migration is a production data change. Treat it as carefully as application code.
