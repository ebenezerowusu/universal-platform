# Monitoring Observability Incident UI Contract

## Purpose

This document defines the MVP UI contract for Monitoring, Observability, and Incident Response foundation.

The screens should help permitted users review service health, alerts, incidents, timelines, runbook references, and post-incident review placeholders.

## Navigation Direction

Monitoring screens should appear when:

- user is authenticated
- organization is selected
- monitoring feature is available
- user has required permission
- feature availability allows operations visibility

## Monitoring Dashboard

Should show:

- overall service status
- latest service check time
- open alerts count
- open incidents count
- degraded services count
- recent incident updates

## Service Status List

Should show:

- service name
- category
- current status
- last checked time
- latency placeholder
- owner placeholder
- action to view detail

## Alert List

Should show:

- alert title/rule
- service
- severity
- status
- triggered time
- resolved time where available
- action to view detail

## Incident List

Should show:

- incident title
- severity
- status
- affected service count
- started time
- resolved time where available
- owner placeholder

## Incident Detail

Should show:

- title
- severity
- status
- safe summary
- affected services
- timeline updates
- runbook reference where available
- post-incident review placeholder

## Incident Timeline

Should show:

- update type
- message
- actor safe label
- timestamp
- status after update where available

## Runbook Reference Placeholder

Should show:

- runbook title
- service/domain
- reference type
- safe link or internal reference placeholder
- status

## Post-Incident Review Placeholder

Should show:

- incident summary
- root cause placeholder
- lessons learned
- follow-up actions
- owner placeholder
- completion status

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 38:

- full Grafana dashboard UI
- external monitoring provider setup UI
- live paging/on-call vendor setup UI
- automatic remediation UI
- script execution UI

## Final Rule

Monitoring UI should make service health and incidents understandable without introducing unsafe automation or provider lock-in.
