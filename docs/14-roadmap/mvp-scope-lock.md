# MVP Scope Lock

## Purpose

This document locks the first MVP scope for Universal Platform.

It exists to prevent scope creep and to keep implementation focused on proving the platform foundation.

## MVP Goal

Build a working platform foundation with one real domain MVP.

The MVP should prove:

- Platform Core works
- capability engines can be used by domains
- modules can be enabled for organizations
- organization-owned data is scoped correctly
- Religious Domain can be built without polluting Core
- Flutter shell can support mobile and desktop/admin paths

The MVP is not the final product.

## MVP Includes: Backend Foundation

The MVP includes:

- Rust backend workspace
- Axum API shell
- configuration foundation
- standard API response format
- standard API error format
- PostgreSQL connection foundation
- Redis foundation where needed
- health endpoint
- first backend tests

## MVP Includes: Platform Core

The MVP includes:

- Identity foundation
- Organization foundation
- organization membership
- Permission Engine foundation
- Audit Engine foundation
- Module Registry foundation
- Subscription or plan feature foundation

## MVP Includes: Engine Foundations

The MVP includes interfaces or local/mock foundations for:

- Payment Engine
- SMS Engine
- Storage Engine
- Notification Engine
- Reporting Engine
- Workflow Engine

Live provider integrations are optional for the first MVP unless explicitly selected later.

## MVP Includes: Religious Domain

The MVP includes:

- Religious module registration
- Religious settings foundation
- member management foundation
- visitor management foundation
- congregations
- services
- groups
- group membership foundation
- manual attendance foundation
- basic Religious reports

## MVP Includes: Flutter Foundation

The MVP includes:

- shared Flutter app shell
- login screen
- organization selector
- mobile shell
- desktop/admin shell foundation
- shared design system foundation
- API client foundation
- loading, empty, and error states

## MVP Excludes

The MVP excludes:

- full marketplace
- full POS sales and inventory
- full Finance module
- full HR module
- logistics and ride-hailing
- QR attendance
- GPS attendance
- complex offline sync
- live multi-provider payment switching
- live multi-provider SMS switching
- full workflow builder
- advanced analytics
- AI insights
- public marketplace mobile experience
- public module marketplace
- external developer SDK
- app store production release automation

## Deferred Religious Features

Defer:

- full family and household module
- full care and counseling module
- advanced branch governance
- advanced giving and pledge reporting
- education program tree attendance
- bulk imports
- advanced communication campaigns
- event ticketing
- public organization page

## Scope Change Rule

A feature can enter MVP only if it meets all conditions:

- it directly proves platform foundation
- it is needed for first usable Religious MVP
- it does not delay Core foundation
- it does not require major provider integration
- it is documented in roadmap and acceptance criteria

## Final Rule

The MVP is not the full product. It is the smallest platform proof that can support real organizations and grow safely.
