# Local Environment Checklist

## Purpose

This document defines the local environment checklist for the first backend implementation.

It helps developers and coding agents confirm the project can run locally before feature work begins.

## Required Local Tools

Confirm local availability of:

- Rust toolchain
- Cargo
- PostgreSQL client or Docker-based PostgreSQL
- Redis client or Docker-based Redis where needed
- Git
- editor or IDE

## Local Services

For MVP development, local services should include:

- PostgreSQL
- Redis
- local file storage folder

Use Docker Compose where practical.

## Local Configuration

Confirm:

- example configuration file exists
- real secrets are not committed
- local placeholders are documented
- backend can read runtime configuration
- missing required values fail clearly

## Database Local Setup

Confirm:

- database can be created locally
- migration command is documented
- migrations can run from a clean database
- local reset process is documented later

## Redis Local Setup

Confirm:

- Redis can start locally
- backend can connect when Redis is required
- queues can use Redis later

## Storage Local Setup

Confirm:

- local storage folder can be created
- file metadata can point to local storage later
- no production storage is required for MVP development

## First Backend Run

Confirm:

- backend starts
- health route responds
- logs show startup clearly
- standard response format is visible

## First Test Run

Confirm:

- backend tests can run locally
- failing tests show useful output
- test command is documented

## Final Rule

Local development must be easy enough that a new developer can run the backend foundation without production services.
