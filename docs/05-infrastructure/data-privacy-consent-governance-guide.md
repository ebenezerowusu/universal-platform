# Data Privacy Consent Governance Guide

## Purpose

This document defines operating guidance for data classifications, sensitive field catalog placeholders, consent purposes, consent records, withdrawal placeholders, access-purpose logs, masking/redaction placeholders, export reviews, subject request placeholders, and privacy audit history.

The goal is to make sensitive data handling consistent, consent-aware, export-aware, and auditable.

## Principle

Sensitive data should be classified, accessed with purpose, exported with review where needed, and governed by consent and retention rules.

## Data Sources

Privacy governance operations may use:

- data classification metadata
- sensitive field catalog placeholder
- consent purpose metadata
- consent record
- consent withdrawal placeholder
- access purpose log placeholder
- masking/redaction policy placeholder
- export review placeholder
- subject request placeholder
- privacy audit history
- audit records

## Classification Direction

Data classifications should capture:

```text
classification_key
title
sensitivity_level_placeholder
description
status
```

## Sensitive Field Direction

Sensitive field catalog records should capture:

```text
domain_module
entity_type
field_key
classification_key
masking_policy_placeholder
export_review_required_placeholder
status
```

## Consent Purpose Direction

Consent purpose records should capture:

```text
purpose_key
title
domain_module_owner
description
retention_linkage_placeholder
status
```

## Consent Record Direction

Consent records should capture:

```text
organization_id
subject_reference_placeholder
purpose_key
status
captured_by
captured_at
source_channel_placeholder
```

## Withdrawal Direction

Withdrawal placeholders should capture:

- consent record
- withdrawal date
- reason optional
- impact review placeholder
- status

Consent withdrawal should not trigger destructive deletion automatically in MVP.

## Access Purpose Logging Direction

Purpose logs should be created where practical for:

- sensitive detail views
- privacy-sensitive exports
- support/admin access
- report generation with sensitive fields
- document access where privacy classification applies

## Masking and Redaction Direction

Masking/redaction placeholders should support:

- UI display masking
- export masking
- report masking
- support view masking
- document metadata masking later

## Export Review Direction

Export review placeholders should check:

- requested export scope
- sensitive field count placeholder
- masking/redaction requirement placeholder
- requesting user permission
- review status

## Subject Request Direction

Subject request placeholders may include:

- access request
- correction request
- export request
- restriction request
- deletion review placeholder

Deletion review should link to retention/archival foundation and should not automatically delete production data.

## Audit Direction

Audit should record:

- classification created or updated
- sensitive field metadata changed
- consent purpose created or updated
- consent captured
- consent withdrawn placeholder
- sensitive record accessed with purpose placeholder
- masking/redaction policy changed
- export review completed placeholder
- subject request updated

## Data Quality Warnings

Warn or flag when:

- sensitive field has no classification
- export includes sensitive fields without review
- consent purpose has no retention linkage
- consent record references missing subject
- withdrawal is pending impact review
- sensitive access occurs without purpose log where required
- masking policy is missing for high-sensitivity classification

## Final Rule

Privacy governance operations must make sensitive data handling explicit, reviewable, and auditable before advanced legal/compliance automation is introduced.
