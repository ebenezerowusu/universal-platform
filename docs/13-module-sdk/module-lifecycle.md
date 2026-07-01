# Module Lifecycle Guide

## Purpose

This document defines the lifecycle of modules in Universal Platform.

A module should have a clear path from registration to installation, activation, updates, suspension, and removal from active use.

## Lifecycle Stages

Recommended stages:

1. Registered
2. Available
3. Installed
4. Configured
5. Active
6. Suspended
7. Inactive
8. Archived

## Registered

The platform knows the module exists.

At this stage, the module has metadata and a manifest but may not be available to organizations yet.

## Available

The module can be installed by eligible organizations.

Availability may depend on:

- platform policy
- region
- subscription plan
- dependencies
- beta access

## Installed

The organization has added the module to its environment.

Installation should:

- check permissions
- check subscription eligibility
- verify dependencies
- create organization module record
- apply default settings
- publish ModuleInstalled event
- record audit entry

## Configured

The organization has set required settings.

Some modules may require configuration before becoming active.

## Active

The module is usable by authorized users.

Feature access still depends on:

- user permissions
- module status
- subscription status
- feature flags
- organization settings

## Suspended

The module is temporarily unavailable.

Reasons may include:

- subscription issue
- platform policy
- dependency issue
- administrative action

## Inactive

The module is installed but not currently usable.

Existing data should remain available according to policy and permissions.

## Archived

The module is no longer actively used for the organization.

Historical records should remain governed by retention and reporting policies.

## Module Updates

Module updates must consider:

- database migrations
- backward compatibility
- permission changes
- setting changes
- event changes
- API changes

Breaking module changes require careful planning and may require an ADR.

## Removal from Active Use

Removing a module from active use should usually disable future actions while preserving historical records according to retention policy.

Do not remove important business history automatically.

## Events Published

- ModuleRegistered
- ModuleAvailable
- ModuleInstalled
- ModuleConfigured
- ModuleActivated
- ModuleSuspended
- ModuleDeactivated
- ModuleArchived
- ModuleUpdated

## Security Requirements

- Module lifecycle actions require permission.
- Organization lifecycle changes must be scoped to the organization.
- Platform-level module changes require platform-level permission.
- Important lifecycle changes must be audited.

## Final Rule

Modules are product capabilities with lifecycle, permissions, settings, and history. Treat them as first-class platform assets.
