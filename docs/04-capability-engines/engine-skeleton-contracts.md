# Engine Skeleton Contracts

## Purpose

This document defines the first skeleton contracts for shared capability engines.

Sprint 4 should create engine boundaries before live provider integrations are implemented.

## Principle

Engines provide reusable capabilities.

Domains should call engines through interfaces, not direct infrastructure or provider code.

## Payment Engine Skeleton

The Payment Engine skeleton should prepare contracts for:

- initialize transaction
- verify transaction
- get transaction status
- handle provider callback later

Sprint 4 should not require a live payment provider.

## SMS Engine Skeleton

The SMS Engine skeleton should prepare contracts for:

- check wallet/balance foundation
- send message request
- get message status
- provider delivery update later

Sprint 4 should not require a live SMS provider.

## Storage Engine Skeleton

The Storage Engine skeleton should prepare contracts for:

- request file upload
- record file metadata
- get file metadata
- generate access path where applicable

Local storage may be used for early implementation.

## Notification Engine Skeleton

The Notification Engine skeleton should prepare contracts for:

- create notification
- list user notifications
- mark notification read
- dispatch through channels later

## Reporting Engine Skeleton

The Reporting Engine skeleton should prepare contracts for:

- register report definition
- run report
- export report later

Religious Domain reports should later use Reporting Engine direction.

## Workflow Engine Skeleton

The Workflow Engine skeleton should prepare contracts for:

- create task
- assign task
- complete task
- list tasks

Full workflow builder is deferred.

## Mock / Local Implementations

Sprint 4 may include mock or local implementations so application services can call engines without live providers.

## Final Rule

Sprint 4 should create engine boundaries, not full provider integrations. Provider-specific implementation belongs behind adapters later.
