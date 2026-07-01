# MVP Scope

## Purpose

This document defines the recommended MVP scope for Universal Platform.

The MVP must prove the platform foundation before expanding into many domains.

## MVP Principle

The MVP should be small enough to build, but strong enough to prove the architecture.

## MVP Foundation Scope

The first MVP foundation should include:

- Backend runtime foundation
- PostgreSQL connection
- Redis connection
- Configuration system
- Standard API responses
- Standard error format
- Identity foundation
- Organization foundation
- Permission foundation
- Audit foundation
- Module Registry foundation
- Subscription foundation
- Basic engine interfaces

## MVP Domain Scope

After platform foundation, the first domain MVP should be the Religious Domain.

Recommended Religious MVP features:

- Religious organization profile/settings
- Member management foundation
- Congregations/services foundation
- Groups foundation
- Basic attendance sessions
- Manual attendance marking
- Visitor registration
- Basic announcements through Notification/SMS engine skeleton
- Basic reports

## Out of MVP

Defer these until foundation and first domain are stable:

- Full marketplace
- Full POS
- Full HR
- Full finance
- Advanced workflow builder
- Advanced analytics
- Third-party module marketplace
- Multiple payment providers
- Complex offline sync
- Full logistics/ride features

## MVP Success Criteria

The MVP succeeds if:

- Platform Core is domain-agnostic.
- Users can belong to organizations.
- Permissions work.
- Tenant isolation is tested.
- Modules can be installed/enabled for organizations.
- Subscription checks can restrict features.
- Audit logs capture sensitive actions.
- First Religious Domain features run on top of the core.
- No provider-specific logic leaks into domains.

## First Revenue-Aligned MVP Features

Revenue-related foundations to prepare early:

- SMS wallet model
- SMS package model
- Payment Engine interface
- Subscription plan model
- Feature/limit checks

Actual provider integrations can come after the skeleton is stable.

## Final Rule

The MVP is not about building every feature. It is about proving the platform model works.
