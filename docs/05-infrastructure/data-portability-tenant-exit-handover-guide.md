# Data Portability Tenant Exit Handover Guide

## Purpose

This document defines operating guidance for export package requests, export scope manifests, handover checklists, exit readiness reviews, ownership confirmations, privacy/lineage linkage, archive/retention linkage, operational links, and portability audit history.

The goal is to make organization data handover controlled, traceable, and non-destructive by default.

## Principle

Tenant exit should be a reviewed portability process, not a raw database operation.

Organizations should receive scoped, reviewed, traceable packages aligned with privacy, retention, billing, and support rules.

## Data Sources

Data portability operations may use:

- export package request
- export scope manifest placeholder
- handover checklist
- exit readiness review placeholder
- ownership confirmation placeholder
- privacy/export review link
- lineage package manifest link
- archive/retention link
- billing/support/offboarding link placeholder
- audit records

## Export Package Direction

Export package requests should capture:

```text
organization_id
requested_by
package_purpose
scope_summary
status
requested_at
```

## Manifest Direction

Export scope manifests should capture:

```text
package_id
included_modules
included_entity_types
excluded_sensitive_categories_placeholder
file_reference_summary_placeholder
generated_through_export_job_placeholder
```

## Handover Checklist Direction

Checklist items should include:

- owner confirmation
- billing status review
- privacy review
- export package review
- support/offboarding review
- archive/retention review

## Exit Readiness Direction

Exit readiness reviews may include:

- unpaid balance warning placeholder
- active subscription warning placeholder
- unresolved support case warning placeholder
- retention hold warning placeholder
- pending export warning placeholder

## Ownership Confirmation Direction

Ownership confirmation should verify that the actor is permitted to approve export and handover operations for the organization.

## Privacy Direction

Portability packages should link to privacy export review where required.

Packages should respect masking/redaction rules and consent-sensitive category warnings.

## Lineage Direction

Package manifests should link to lineage summaries where practical so recipients can understand source, import, export, and transformation metadata.

## Archive and Retention Direction

Portability should link to archive and retention placeholders without triggering destructive deletion automatically.

## Operational Link Direction

Portability workflows may link to:

- billing status
- subscription status
- support cases
- offboarding status
- organization lifecycle status

## Audit Direction

Audit should record:

- export package requested
- export package approved/rejected placeholder
- manifest generated placeholder
- ownership confirmation captured placeholder
- handover checklist updated
- exit readiness reviewed placeholder
- privacy review linked
- lineage manifest linked
- archive/retention linkage changed

## Data Quality Warnings

Warn or flag when:

- export package has no privacy review where required
- package manifest is incomplete
- lineage manifest is missing for important datasets
- billing or support blockers are unresolved
- retention hold conflicts with exit request
- ownership confirmation is missing
- package contains unresolved data quality warnings

## Final Rule

Data portability operations must package organization data safely, preserve auditability, and avoid destructive exit shortcuts.
