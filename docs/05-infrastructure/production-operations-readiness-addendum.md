# Production Operations Readiness Addendum

## Purpose

This document closes the remaining production-operations documentation gaps for Universal Platform.

It covers frontend delivery, APIs/backend logic, database/storage, authentication and permissions, hosting/deployment, cloud/compute evolution, CI/CD, security and RLS direction, rate limiting, caching/CDN, load balancing/scaling, error tracking/logs, and availability/recovery.

This is an operations addendum. Detailed implementation still belongs in code, migrations, deployment scripts, and sprint implementation logs.

## Status

```text
architecture documentation track complete
production operations coverage added
implementation should start now
```

## 1. Frontend Operations

Initial frontend stack:

```text
Flutter mobile
Flutter desktop
```

Frontend rules:

- render based on authenticated user, selected organization, installed modules, permissions, feature flags, and localization settings
- keep business logic out of Flutter UI
- support role-based routing
- support module-aware navigation
- support platform-specific layouts for mobile and desktop
- support future offline-first workflows for attendance and field operations where required

Future frontend delivery may include:

```text
Web portal placeholder
Public pages placeholder
Partner/developer portal placeholder
```

Frontend builds should later have separate release channels:

```text
development
staging
production
```

## 2. APIs And Backend Logic

Initial backend stack:

```text
Rust
Axum
Tokio
PostgreSQL
Redis
Docker Compose
```

API rules:

- version every API under `/api/v1`
- keep handlers thin
- validate input server-side
- resolve tenant/organization context before data access
- call application services instead of embedding business logic in handlers
- return consistent response shapes
- include trace IDs in responses where practical
- do not expose stack traces, SQL queries, secrets, or internal paths

Backend logic placement:

```text
API handlers -> Application services -> Domain services / Capability engines -> Repositories / Adapters
```

Forbidden:

```text
business logic inside API handlers
direct provider calls inside domains
domain-specific concepts inside Platform Core
```

## 3. Database And Storage

Primary database:

```text
PostgreSQL
```

Initial storage:

```text
Local VPS storage through Storage Engine
```

Future storage:

```text
S3-compatible object storage
Cloudflare R2
Azure Blob Storage
future object storage provider
```

Database rules:

- use migrations for every schema change
- use tenant-aware tables for organization-owned data
- include `organization_id` on tenant-owned records
- use UUID/ULID public identifiers where practical
- add indexes around organization, status, dates, and common reports
- use JSONB only for appropriate flexible metadata
- avoid using JSONB for core relational concepts
- maintain auditability for critical changes

Storage rules:

- use Storage Engine only
- never store raw provider credentials in domain tables
- validate file type and size
- generate safe storage names
- enforce tenant ownership
- move to object storage through configuration when required

## 4. Auth And Permissions

Authentication should support:

- email/password
- secure password hashing
- password reset
- session/token management
- future OAuth/OpenID
- future MFA for sensitive roles

Authorization must use Permission Engine.

Authorization must consider:

```text
user identity
organization context
role
permission
scope
installed modules
subscription limits
resource ownership
```

Frontend permissions are hints only.

Backend permission checks are the source of truth.

## 5. Hosting And Deployment

Initial deployment target:

```text
Hostinger VPS or equivalent VPS
Ubuntu 24 LTS
Docker Compose
Nginx
Rust + Axum API
PostgreSQL
Redis
Background workers
Local file storage
Let's Encrypt SSL
```

Initial flow:

```text
Internet
  -> Nginx
    -> Rust + Axum API
      -> PostgreSQL
      -> Redis
      -> Workers
      -> Storage Engine
```

Deployment rules:

- use HTTPS everywhere
- do not expose PostgreSQL publicly
- do not expose Redis publicly
- use firewall rules
- keep secrets outside git
- use least-privilege service users
- use environment variables or secret management

## 6. Cloud And Compute Evolution

The platform starts on one VPS, but must be designed to move services without changing application code.

Compute evolution stages:

```text
Stage 1: Single VPS
Stage 2: Dedicated PostgreSQL server
Stage 3: Dedicated Redis server
Stage 4: Separate worker nodes
Stage 5: Object storage/CDN
Stage 6: Multiple API nodes behind load balancer
Stage 7: Managed services or cloud-native deployment where justified
```

Configuration must control service locations:

```env
DATABASE_URL=...
REDIS_URL=...
STORAGE_PROVIDER=...
QUEUE_PROVIDER=...
```

Application code should not care whether PostgreSQL, Redis, storage, or workers are local, dedicated, or managed.

## 7. CI/CD And Version Control

Version control:

```text
GitHub
main branch protected where practical
feature/fix/chore/docs branches for work
pull requests for implementation changes
```

Initial CI should run:

```text
cargo fmt
cargo check
cargo test
```

Future CI/CD should add:

```text
migration validation
security checks
container build
staging deployment
smoke tests
production approval gate
rollback/recovery notes
Flutter analyze/test/build checks
```

Deployment should not be fully automatic for production until:

- backups are verified
- migrations are tested
- health checks exist
- rollback/recovery plan exists
- environment variables are reviewed
- secrets are managed safely

## 8. Security And RLS Direction

Security principles:

```text
secure by default
least privilege
tenant isolation
defense in depth
audit sensitive actions
never trust the client
never expose provider secrets
validate all input
protect sensitive data
```

Tenant isolation is mandatory.

Required baseline enforcement:

- application service authorization
- repository-level tenant filters
- permission checks
- API handler organization context resolution
- audit logs
- tenant isolation tests

### PostgreSQL RLS Direction

PostgreSQL Row-Level Security may be introduced after the first database foundations are stable.

Initial MVP decision:

```text
Use application-service and repository-level tenant isolation first.
Design schemas so PostgreSQL RLS can be added later where beneficial.
```

RLS should be considered for high-risk tables such as:

```text
financial records
member/contact records
staff records
confidential counseling or prayer records
payment metadata
data export records
partner delegated access records
```

RLS adoption requires an ADR before implementation.

The ADR should decide:

- which tables use RLS
- how organization context is set per DB session/transaction
- how background jobs set tenant context
- how migrations and admin operations bypass or respect policies safely
- how tests verify RLS policies

Until RLS exists, missing tenant filters are critical bugs.

## 9. Rate Limiting

Rate limiting should protect:

```text
login
password reset
OTP/MFA placeholder
SMS sending
Email sending
WhatsApp sending
payment initialization
public endpoints
webhooks
imports
exports
search
bulk messaging
```

Initial implementation may use Redis-backed counters.

Rate limit dimensions may include:

```text
IP address
user ID
organization ID
route/action
provider/channel
```

Rate limit failures should return safe structured errors and should not reveal internal thresholds where sensitive.

## 10. Caching And CDN

Initial caching:

```text
Redis
```

Initial cache use cases:

- sessions or token revocation placeholders
- rate limiting counters
- feature flag/config evaluation cache
- dashboard summary cache where safe
- queue/job state where appropriate

Cache rules:

- never cache sensitive data without explicit review
- include tenant context in cache keys
- define TTLs
- invalidate cache on important writes
- avoid stale permission or entitlement decisions

CDN direction:

CDN should be introduced when static and media delivery grows.

Future CDN use cases:

```text
public assets
organization logos
public event/media assets
marketplace product images
help center images
large static downloads where safe
```

CDN rules:

- private files must not be public CDN objects
- signed URLs should be used for restricted files
- object storage should become the origin for media at scale
- cache-control headers must match privacy level
- tenant-private media must remain permission-protected

## 11. Load Balancing And Scaling

Before multiple API nodes:

- API must be stateless
- sessions must be Redis-backed or token-based
- file storage must be shared/object storage
- logs must be centralized
- health/readiness checks must exist
- background workers must be separate from request handling where required

Scaling stages:

```text
single VPS
separate DB
separate Redis
separate workers
object storage/CDN
multiple API nodes
load balancer
managed services where justified
```

Load balancer requirements:

- health checks
- HTTPS termination or pass-through strategy
- request IDs preserved
- sane timeout settings
- upload size limits
- rate limiting at edge/reverse proxy where useful

## 12. Error Tracking And Logs

Minimum logging from day one:

```text
application logs
error logs
request IDs / trace IDs
health check status
DB connection failures
Redis connection failures
worker failures
payment/SMS/Email/WhatsApp provider errors
webhook verification failures
background job failures
```

Log rules:

- logs must not include passwords, tokens, private keys, or raw provider secrets
- logs must avoid full sensitive payloads
- include organization ID only where safe and useful
- include correlation/trace IDs
- classify severity

Future error tracking options:

```text
Sentry placeholder
Bugsnag placeholder
OpenTelemetry placeholder
Prometheus/Grafana placeholder
Loki/ELK placeholder
```

The first production deployment should at least provide:

- application logs
- process logs
- Nginx access/error logs
- database backup logs
- worker logs
- deployment logs

## 13. Availability And Recovery

Minimum availability direction:

```text
MVP: best-effort availability with daily backups
Early production: documented recovery process and monitored health checks
Growth stage: defined uptime target, RTO, and RPO
```

Initial recovery requirements:

- daily PostgreSQL backups
- backup retention policy
- backup verification
- off-server backup storage as soon as practical
- uploaded file backup while local storage is used
- restore documentation
- deployment rollback notes

Suggested targets to define before serious production rollout:

```text
Uptime target placeholder: 99.5% early production
RPO placeholder: 24 hours until tighter backup schedule exists
RTO placeholder: 4-8 hours until tested restore automation exists
```

These targets must be reviewed before being promised to customers.

## 14. Production Readiness Checklist

Before production launch:

```text
[ ] HTTPS configured
[ ] firewall configured
[ ] PostgreSQL not public
[ ] Redis not public
[ ] secrets outside git
[ ] environment variables reviewed
[ ] database backups enabled
[ ] restore test documented
[ ] health endpoint working
[ ] readiness endpoint working
[ ] application logs available
[ ] worker logs available
[ ] Nginx logs available
[ ] rate limiting enabled for sensitive routes
[ ] tenant isolation tests passing
[ ] permission tests passing
[ ] audit events implemented for sensitive actions
[ ] deployment notes created
[ ] rollback/recovery notes created
```

## 15. What Remains Implementation-Driven

Do not over-document before code.

Create additional notes only when implementation begins for:

- exact CI/CD workflow files
- exact Nginx config
- exact Docker Compose production file
- exact database backup script
- exact RLS ADR if adopted
- exact monitoring tool selection
- exact CDN provider selection
- exact deployment runbook

## Final Rule

The platform is production-aware, but implementation should still start simple.

Start on one VPS. Preserve clean boundaries. Scale by configuration and adapters, not rewrites.
