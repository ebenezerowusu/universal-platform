# Religious Custom Fields and Forms Specification

## Purpose

This document defines custom fields and dynamic forms for Religious Domain records.

Custom fields allow organizations to collect additional information without changing the database schema for every organization.

## Scope

Custom fields may apply to:

- Members
- Visitors
- Groups
- Events
- Prayer requests
- Welfare requests
- Counseling intake forms
- Education program registration

## Field Types

Supported field types may include:

- text
- textarea
- number
- phone
- email
- date
- datetime
- dropdown
- radio
- checkbox
- multi_select
- file
- image
- yes_no

## Field Definition Properties

A field definition may include:

- label
- key
- type
- required
- placeholder
- help_text
- default_value
- validation_rules
- visibility
- editable_by_member
- editable_by_admin
- section
- sort_order
- active

## Form Sections

Common sections:

- Personal Information
- Family Information
- Spiritual Information
- Emergency Contact
- Organization Information
- Admin Only

Labels should be localization-ready.

## Suggested Entities

Candidate entities:

- religious_custom_field_definitions
- religious_custom_field_values
- religious_form_definitions
- religious_form_sections
- religious_form_fields

## Storage Pattern

Core relational fields should remain in normal columns.

Custom fields should store additional flexible data.

Do not use custom fields to avoid modeling important platform concepts.

## Required Platform Engines

This module must use:

- Organization Engine for tenant context
- Permission Engine for access control
- Storage Engine for file/image fields
- Audit Engine for sensitive changes
- Localization Engine for labels

## Permission Examples

- religious.forms.view
- religious.forms.manage
- religious.custom_fields.view
- religious.custom_fields.manage

## Security Requirements

- Sensitive custom fields must have visibility rules.
- File fields must use Storage Engine.
- Admin-only fields must not appear in member self-service forms.
- Custom field values are organization-owned.

## Testing Requirements

Tests must cover:

- Create custom field
- Attach field to form
- Save custom field value
- Validate required field
- Hide admin-only field from member view
- Permission checks
- Organization boundary checks

## Final Rule

Custom fields add flexibility. They must not replace proper modeling for core business concepts.
