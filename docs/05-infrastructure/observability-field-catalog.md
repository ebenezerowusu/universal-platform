# Observability Field Catalog

## Purpose

This document defines common fields for logs, traces, jobs, provider records, and operational diagnostics.

Consistent observability fields make support, debugging, and incident review easier.

## Principle

Operational records should help explain what happened without exposing sensitive data unnecessarily.

## Common Fields

Recommended common fields:

```text
trace_id
correlation_id
request_id
organization_id
actor_user_id
service
module
operation
status
error_code
duration_ms
created_at
```

## HTTP Request Logs

Useful fields:

```text
method
path
status_code
duration_ms
trace_id
organization_id
actor_user_id
user_agent
ip_address
```

Do not log full sensitive request bodies.

## Application Service Logs

Useful fields:

```text
operation
resource_type
resource_id
organization_id
actor_user_id
result
error_code
trace_id
```

## Worker Job Logs

Useful fields:

```text
job_id
job_type
status
attempt
max_attempts
organization_id
trace_id
error_code
duration_ms
```

## Provider Logs

Useful fields:

```text
provider_key
provider_type
operation
reference
status
error_code
organization_id
trace_id
duration_ms
```

Do not log provider secrets.

## Audit vs Logs

Logs support operations and debugging.

Audit records support accountability.

Do not assume logs replace audit records for sensitive actions.

## Final Rule

Observability should make the platform understandable in production without leaking sensitive information.
