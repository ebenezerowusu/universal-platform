# Religious MVP Acceptance Criteria

## Purpose

This document defines completion criteria for the first Religious MVP foundation.

The Religious MVP should prove that a domain can be built on top of Platform Core and shared engines without breaking architectural boundaries.

## Module Criteria

Religious MVP modules are ready when:

- required module records exist
- modules can be added to an organization
- module availability checks work
- Religious use cases require the correct module availability

## Settings Criteria

Religious settings are ready when:

- organization-level Religious settings can be stored
- default settings exist where useful
- settings are read by Religious use cases where needed
- Platform Core remains unaware of Religious terminology

## Member Criteria

Member foundation is ready when:

- members can be created
- members can be listed
- member detail can be viewed
- member can be updated
- status history foundation exists
- member queries use selected organization context

## Structure Criteria

Structure foundation is ready when:

- congregations can be created and listed
- services can be created and listed
- groups can be created and listed
- members can be assigned to groups where planned

## Visitor Criteria

Visitor foundation is ready when:

- visitors can be created
- visitors can be listed
- visitor follow-up foundation exists
- visitor can be converted to member through an application service

## Attendance Criteria

Attendance foundation is ready when:

- attendance sessions can be created
- manual attendance records can be marked
- duplicate attendance behavior is handled safely
- attendance queries use selected organization context

## Report Criteria

Basic reports are ready when:

- member count report exists
- visitor count report exists
- group count report exists
- attendance summary foundation exists
- reports use Reporting Engine direction

## Architecture Criteria

Religious MVP must:

- use Identity for user context
- use Organization for organization context
- use Permission Engine for actions
- use Audit Engine for important changes
- use Module Registry for module checks
- use engine interfaces for shared capabilities
- keep Religious business concepts inside Religious Domain

## Out of Scope

Not required for first Religious MVP foundation:

- QR attendance
- GPS attendance
- full giving module
- live payment provider
- live SMS provider
- full family/household module
- full care/counseling module
- offline sync

## Final Rule

Religious MVP is accepted only when it proves the platform architecture works for a real domain.
