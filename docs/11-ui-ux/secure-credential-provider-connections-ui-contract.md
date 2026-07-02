# Secure Credential Provider Connections UI Contract

## Purpose

This document defines the MVP UI contract for Secure Credential References, Provider Connections, and Environment Profiles foundation.

The screens should help permitted users review provider access references, provider connections, environment profiles, rotation placeholders, connection health, capability mappings, access review placeholders, and usage audit history.

## Navigation Direction

Provider connection screens should appear when:

- user is authenticated
- organization is selected
- provider connection feature is available
- user has required permission
- feature availability allows provider connection visibility

## Provider Connection Dashboard

Should show:

- provider connection count
- active connection count
- health warning count
- rotation review due count
- environment profile count
- recent provider connection events

## Access Reference List

Should show:

- reference key
- provider type
- provider key
- environment profile
- status
- last rotation placeholder
- action to view detail

Sensitive access material should not appear in normal UI views.

## Provider Connection List

Should show:

- provider type
- provider key
- environment profile
- access reference safe label
- capability summary
- health status
- status

## Provider Connection Detail

Should show:

- provider metadata
- regional compatibility summary
- capability mapping summary
- access reference metadata
- health check summary
- usage audit summary
- disable placeholder action where permitted

## Environment Profile List

Should show:

- profile key
- display name
- purpose
- status
- warning count where available

## Rotation Request Placeholder

Should show:

- access reference safe label
- requested by safe label
- status
- requested date
- approved date placeholder
- completed date placeholder
- safe notes

## Connection Health Summary

Should show:

- provider connection safe label
- health status
- last checked date
- safe failure summary
- warning count
- action required placeholder

## Capability Mapping Summary

Should show:

- provider type
- provider key
- capability type
- country/currency linkage where available
- status
- warning count

## Usage Audit History

Should show:

- event type
- actor/system safe label
- target safe label
- timestamp
- safe metadata summary

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 49:

- sensitive access material display UI
- arbitrary provider command UI
- provider-specific SDK setup UI inside domains
- cross-tenant provider connection browser UI
- automatic rotation setup UI without approval controls

## Final Rule

Provider connection UI should expose safe operational metadata while keeping sensitive provider access hidden behind controlled references.
