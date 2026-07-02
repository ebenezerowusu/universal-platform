# Data Lineage Provenance Record History Guide

## Purpose

This document defines operating guidance for source systems, provenance records, record history placeholders, import/job/report/export lineage, transformation steps, derived relationships, lineage graph summaries, correction/merge lineage, and lineage audit history.

The goal is to make data origin and change relationships understandable and trustworthy.

## Principle

Lineage explains source and relationship metadata.

It should not claim to reconstruct every historical state unless the underlying record history supports that claim.

## Data Sources

Lineage operations may use:

- source system metadata
- record provenance metadata
- record version/history placeholder
- lineage link
- transformation step placeholder
- derived record relationship placeholder
- lineage graph summary placeholder
- correction/merge lineage link
- audit records

## Source System Direction

Source systems should capture:

```text
source_key
source_type
owner_module_domain
trust_level_placeholder
status
```

## Provenance Direction

Provenance records should capture:

```text
organization_id
entity_reference
source_system_key
source_record_reference_placeholder
created_by_process_or_actor
created_through_placeholder
created_at
```

## Record History Direction

Record history placeholders should capture:

```text
organization_id
entity_reference
version_placeholder
changed_fields_safe_summary
changed_by_safe_label
change_source
reason_placeholder
created_at
```

Avoid storing full sensitive before/after values unless a future security-reviewed versioning design supports it.

## Linkage Direction

Lineage links may connect records to:

- import jobs
- async job runs
- report snapshots
- export requests
- migration batches
- webhook or integration events
- correction requests
- merge reviews
- canonical markers

## Transformation Direction

Transformation metadata may include:

- field mapping
- normalization placeholder
- validation rule application
- correction application
- merge review output placeholder

## Derived Relationship Direction

Derived relationships may include:

- report row derived from source record
- export derived from report snapshot
- canonical marker derived from duplicate review
- search index derived from entity record
- aggregated metric derived from multiple source records

## Privacy Direction

Lineage views should respect privacy governance:

- mask sensitive field names or summaries where required
- avoid exposing sensitive values in graph summaries
- apply access-purpose logging where required
- hide lineage nodes the actor cannot access

## Data Quality Direction

Data quality operations should create lineage links where practical for:

- duplicate candidate creation
- merge review decision
- canonical marker change
- correction request completion
- validation result resolution

## Audit Direction

Audit should record:

- source system created or updated
- provenance record created placeholder
- lineage link created or updated
- transformation metadata changed
- derived relationship changed
- record history reviewed placeholder
- correction/merge lineage reviewed placeholder

## Data Quality Warnings

Warn or flag when:

- important record has unknown source
- lineage link points to missing source reference
- report/export has incomplete lineage
- correction has no lineage link
- derived relationship conflicts with domain ownership
- graph summary hides nodes due to privacy restrictions

## Final Rule

Lineage operations must explain data origin and derivation while preserving privacy, tenant isolation, and domain authority.
