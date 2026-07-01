# API Verification Plan

## Purpose

This document defines API verification direction for Universal Platform.

Verification confirms that implemented routes match documented API contracts.

## Principle

API verification should happen at the route boundary.

It should confirm that clients receive consistent platform responses.

## Verification Areas

Verify:

- route path
- method
- request body shape
- query parameters
- success response shape
- error response shape
- pagination shape
- created resource response
- update response
- report response

## Core API Verification

Verify:

- health route
- auth routes
- organization routes
- module routes
- subscription routes
- audit routes

## Engine API Verification

Verify when implemented:

- payment routes
- SMS routes
- storage routes
- notification routes
- reporting routes
- workflow routes

## Religious API Verification

Verify when implemented:

- member routes
- visitor routes
- congregation routes
- service routes
- group routes
- attendance routes
- report routes

## Contract Review

For each route, confirm:

- documented request matches implementation
- documented response matches implementation
- field names are consistent
- errors use the platform error wrapper
- trace metadata is present where required

## Final Rule

API verification protects the contract between backend and clients.
