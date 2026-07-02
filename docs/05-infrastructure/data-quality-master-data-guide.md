# Data Quality Master Data Guide

## Purpose

This document defines operating guidance for data quality rules, validation results, duplicate detection, duplicate candidates, safe merge reviews, master data registry placeholders, canonical markers, correction requests, and data quality audit history.

The goal is to improve tenant data accuracy without destructive automation.

## Principle

Data quality operations should identify issues, guide review, and preserve audit history before any corrective action is applied.

## Data Sources

Data quality operations may use:

- data quality rule metadata
- validation result metadata
- duplicate detection rule placeholder
- duplicate candidate
- merge review placeholder
- master data registry placeholder
- canonical record marker placeholder
- data quality score placeholder
- correction request placeholder
- audit records

## Rule Direction

Data quality rules should capture:

```text
rule_key
module_domain_owner
entity_type
rule_type
severity_placeholder
status
```

## Validation Result Direction

Validation results should capture:

```text
organization_id
rule_key
entity_reference
status
message_safe_summary
detected_at
```

## Duplicate Rule Direction

Duplicate rules may include:

- exact match
- fuzzy match placeholder
- phone/email match
- identity/reference match
- cross-field match placeholder

## Duplicate Candidate Direction

Duplicate candidates should capture:

```text
organization_id
entity_type
primary_record_reference
candidate_record_reference
confidence_placeholder
status
detected_by_rule
```

## Merge Review Direction

Merge review placeholders should capture:

- candidate reference
- field comparison safe summary
- privacy-sensitive field warning
- merge plan placeholder
- review status
- reviewer safe label

Merge reviews should not delete duplicate records automatically in MVP.

## Master Data Registry Direction

Master data registry placeholders may link:

- person/member/customer/worker records
- organization contact records
- product/item records
- location/address records

## Canonical Marker Direction

Canonical markers identify preferred records without deleting alternates.

Use canonical markers to guide reporting, search, exports, and future merge workflows.

## Correction Request Direction

Correction requests may include:

- invalid data
- missing data
- conflicting data
- duplicate data
- privacy-sensitive correction review

## Privacy Direction

Data quality views should respect privacy governance:

- apply masking where required
- avoid exposing sensitive fields in duplicate review summaries
- log access purpose where required
- flag privacy-sensitive merge reviews

## Audit Direction

Audit should record:

- data quality rule created or updated
- validation result reviewed
- duplicate candidate reviewed
- merge review approved/rejected placeholder
- canonical record marker changed
- correction request created or completed
- bulk correction placeholder reviewed

## Data Quality Warnings

Warn or flag when:

- duplicate candidate has high confidence
- validation result remains open too long
- canonical marker conflicts with source domain rules
- merge review contains privacy-sensitive fields
- correction request is overdue placeholder
- import has unresolved validation results
- report/search uses records with open quality warnings

## Final Rule

Data quality operations must preserve tenant ownership, privacy masking, audit history, and domain authority while improving record trust.
