# Sprint 1 Review Checklist

## Purpose

This checklist helps reviewers evaluate the first backend scaffold PR.

It is designed for both human reviewers and coding-agent reviewers.

## Scope Review

Confirm the PR includes only Sprint 1 scope:

- backend workspace
- API shell
- runtime configuration foundation
- standard response model
- standard error model
- health endpoint
- first tests

Confirm the PR does not include:

- Identity implementation
- Organization implementation
- Permission Engine implementation
- Audit Engine implementation
- Religious Domain implementation
- provider integration
- Flutter UI

## Architecture Review

Confirm:

- folder structure follows documented target direction
- API layer stays thin
- shared types are reusable
- infrastructure concerns are placed appropriately
- no domain-specific concepts appear in Core

## API Review

Confirm:

- health route follows `docs/12-api/health-api-contract.md`
- standard response follows `docs/12-api/standard-response-contract.md`
- errors use standard shape where applicable

## Configuration Review

Confirm:

- runtime configuration has a clear loading path
- local defaults or examples are safe
- missing required values are handled clearly where applicable

## Test Review

Confirm:

- health endpoint test exists
- response shape test exists or is covered
- tests can run locally
- PR summary lists checks run

## Documentation Review

Confirm:

- implementation matches current docs
- docs were updated if implementation differed
- PR uses the pull request template

## Merge Recommendation

Approve only when:

- build passes
- tests pass
- scope is controlled
- architecture boundaries are respected
- known limitations are documented

## Final Rule

The first PR sets the tone for the whole platform. Review it carefully and keep it small.
