# Platform Glossary

## Purpose

This glossary defines shared language for Universal Platform.

Consistent language helps developers, coding agents, architects, and product contributors avoid confusion as the platform grows.

## Platform

The full Universal Platform product and ecosystem.

It includes Platform Core, capability engines, domains, modules, clients, APIs, infrastructure, and operational tooling.

## Platform Core

The domain-agnostic foundation of the platform.

Core knows generic concepts such as:

- users
- organizations
- permissions
- modules
- subscriptions
- settings
- audit

Core must not know domain-specific concepts such as church members, products, orders, staff payroll, or drivers.

## Organization

A tenant or customer account using the platform.

An organization may be a church, ministry, shop, school, company, nonprofit, or another real-world group.

## Tenant

Another word for organization in multi-tenant architecture.

In this project, `organization` is the preferred product term.

## Organization User

A user connected to an organization.

A single global user may belong to multiple organizations.

## Domain

A business area built on top of Platform Core and Engines.

Examples:

- Religious
- Commerce
- POS
- Finance
- HR
- Logistics later

## Module

An installable or activatable unit of platform functionality.

A module may belong to a domain or engine.

Examples:

- religious.members
- religious.attendance
- sms
- payments
- pos.sales

## Capability Engine

A reusable platform capability used by many domains.

Examples:

- Payment Engine
- SMS Engine
- Storage Engine
- Notification Engine
- Reporting Engine
- Workflow Engine

Engines provide capability. Domains provide business meaning.

## Provider Adapter

A replaceable integration layer for an external service.

Examples:

- SMS provider adapter
- payment provider adapter
- storage provider adapter

Domains must not call providers directly.

## Permission

A stable action key that controls what a user can do.

Example:

```text
religious.members.create
```

## Role

A group of permissions assigned to organization users.

Roles are organization-specific unless explicitly platform-level.

## Permission Scope

The boundary within which a permission applies.

Examples:

- own
- own_group
- branch
- subtree
- organization
- platform

## Subscription

The active commercial plan or package assigned to an organization.

Subscription controls feature access and limits together with Module Registry and Feature/License checks.

## Feature Flag

A controlled switch that enables or disables a feature for an environment, organization, group, or rollout.

## Configuration

Settings that control behavior.

Configuration may exist at platform, organization, module, or feature level.

## Branch

In the Religious Domain, a branch is an organization or sub-organization within a religious hierarchy.

Branch hierarchy belongs to the Religious Domain, not Platform Core.

## Congregation

In the Religious Domain, a congregation represents a people grouping.

Examples:

- Adults
- Youth
- Teens
- Children
- Language-specific congregation

## Service

In the Religious Domain, a service represents a scheduled gathering.

Examples:

- First Service
- Second Service
- Midweek Service
- Prayer Service

## Group

In the Religious Domain, a group is a smaller unit within a congregation.

Examples:

- Department
- Cell
- Study group
- Fellowship
- Ministry team

## Member

In the Religious Domain, a member is a person connected to the religious organization.

A member may or may not have a platform login account.

## Visitor

In the Religious Domain, a visitor is a person who has visited or shown interest but is not yet a member.

## Attendance Session

A record of an event, service, group meeting, or program where attendance can be marked.

## Attendance Record

A record that marks one person's attendance status for one session.

## Workflow

A configurable process with steps, tasks, assignments, reminders, and status changes.

Examples:

- visitor follow-up
- finance approval
- welfare case review

## Audit Log

A record of important actions for accountability.

Audit is different from general application logs.

## Timeline Entry

A user-friendly historical event shown on a record's activity timeline.

Timeline helps users understand history. Audit helps the platform prove accountability.

## Final Rule

When a term is unclear, define it here before using it broadly in implementation.
