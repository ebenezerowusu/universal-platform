# Data Retention and Deletion Policy Direction

## Purpose

This document defines the first data retention and deletion policy direction for Universal Platform.

It is an architectural policy direction, not final legal advice.

## Principle

Data should be retained only as long as needed for product, operational, financial, legal, audit, or support reasons.

Deletion behavior must be intentional, traceable, and safe.

## User Records

User records may need to be retained when they are connected to organizations, audit logs, transactions, or historical activity.

Recommended direction:

- support account deactivation
- support profile data minimization where needed
- avoid deleting records that break audit or financial history

## Organization Records

Organization records should not be hard-deleted casually.

Recommended direction:

- support suspended, disabled, and archived states
- keep historical records where needed for audit and financial integrity
- define export and closure process later

## Audit Logs

Audit logs should be retained for accountability.

Recommended direction:

- do not edit audit records casually
- avoid storing unnecessary sensitive values inside audit metadata
- define retention duration before production launch

## Payment Records

Payment records may need longer retention because of reconciliation, receipts, disputes, and financial reporting.

Recommended direction:

- keep transaction records
- avoid deleting transaction history casually
- hide sensitive provider details from general users

## SMS Records

SMS records may be needed for usage history, delivery review, billing, and support.

Recommended direction:

- keep wallet transaction records
- keep message status records for operational review
- avoid storing unnecessary message content longer than needed where future policy requires minimization

## File Records

File records include metadata and stored file content.

Recommended direction:

- support soft delete or archived status
- remove physical files only through controlled storage workflows
- avoid orphaning file metadata or file content

## Religious Member Records

Religious member records may include personal and sensitive information.

Recommended direction:

- support inactive, transferred, deceased, and archived statuses
- avoid hard delete as the default behavior
- restrict sensitive fields by permission
- audit sensitive exports or sensitive profile access where needed

## Visitor Records

Visitor records should support lifecycle transitions.

Recommended direction:

- new
- contacted
- interested
- converted
- inactive
- archived

Define later whether old inactive visitor records should be anonymized or archived.

## Care, Counseling, and Welfare Records

These records may be sensitive.

Recommended direction:

- stricter access rules
- limited visibility
- careful audit
- clear retention policy before production use

## Backups

Backups may retain deleted or archived data for a period of time.

Backup retention should be documented in infrastructure policy before production launch.

## Final Rule

Deletion must not be an accidental technical action. It must follow platform policy, preserve required history, and protect sensitive information.
