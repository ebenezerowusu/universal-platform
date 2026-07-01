# Code Review Checklist

## Purpose

This checklist helps reviewers protect Universal Platform's architecture, security, and quality.

Every implementation PR should be reviewed against these checks.

## Architecture Checks

- Does the change respect the Platform Constitution?
- Is the Platform Core still domain-agnostic?
- Is business logic in the correct layer?
- Are API handlers thin?
- Are domains separated from engines and infrastructure?
- Are external providers accessed only through engines/adapters?
- Is the change configurable where it should be?

## Multi-Tenancy Checks

- Are organization-owned records scoped by organization?
- Do database queries include tenant filters?
- Can a user from Organization A access Organization B data?
- Are tests included for tenant isolation?

## Permission Checks

- Does every protected action enforce permission checks?
- Is scope considered where needed?
- Is module availability checked where needed?
- Is subscription access checked for restricted features?
- Is frontend hiding supported by backend authorization?

## Security Checks

- Are secrets excluded from code and logs?
- Are sensitive actions audited?
- Are errors safe for clients?
- Are provider responses sanitized?
- Are file uploads validated where relevant?
- Are webhooks or callbacks verified where relevant?

## Database Checks

- Are migrations included where needed?
- Are foreign keys used where appropriate?
- Are indexes considered for expected queries?
- Are unique constraints scoped correctly?
- Is JSONB used only where appropriate?
- Does the migration protect existing data?

## API Checks

- Does the API follow response/error standards?
- Are error codes stable?
- Are pagination/filtering patterns consistent?
- Should OpenAPI documentation be updated?

## Testing Checks

- Are unit tests included where needed?
- Are integration tests included where needed?
- Are permission tests included?
- Are tenant isolation tests included?
- Were provider interactions mocked where appropriate?

## Documentation Checks

- Does this change require documentation updates?
- Does it require an ADR?
- Are module/domain/engine docs still accurate?

## AI Agent Checks

If an AI coding agent made the change:

- Did it read the required docs?
- Did it classify the task correctly?
- Did it explain architecture impact?
- Did it avoid unrelated changes?

## Final Rule

Do not approve code that makes the platform harder to extend, secure, test, or operate.
