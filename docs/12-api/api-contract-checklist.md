# API Contract Checklist

## Purpose

This document defines a checklist for reviewing Universal Platform API routes.

The goal is to keep API behavior consistent across Core, Engines, and Domains.

## Route Checklist

For each route, document:

- route path
- HTTP method
- request body
- query parameters
- response data shape
- pagination shape where applicable
- related module where applicable
- related permission where applicable
- related events where applicable

## Response Checklist

Each route should use:

- standard success wrapper
- standard error wrapper
- trace metadata
- consistent field naming

## List Route Checklist

List routes should define:

- pagination defaults
- maximum page size
- allowed filters
- allowed sort fields
- search behavior where applicable

## Create Route Checklist

Create routes should define:

- required fields
- optional fields
- validation expectations
- created record response
- related event where applicable

## Update Route Checklist

Update routes should define:

- editable fields
- non-editable fields
- state change behavior
- audit expectation where applicable

## Report Route Checklist

Report routes should define:

- report key
- filters
- output format
- export behavior where applicable
- background job behavior where applicable

## Engine Route Checklist

Engine routes should define:

- capability being requested
- provider-hidden behavior
- engine response shape
- related internal event where applicable

## Final Rule

Every API route should have a clear contract before implementation becomes permanent.
