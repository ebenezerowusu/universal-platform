# Localization Engine Specification

## Purpose

The Localization Engine defines how Universal Platform handles display language, regional settings, timezones, currencies, date formats, and organization-specific labels.

The goal is to keep the Platform Core flexible for many countries and organization types.

## Responsibilities

The Localization Engine is responsible for:

- Supported languages
- Translation keys
- Translation values
- Regional settings
- Timezone settings
- Currency settings
- Date and time display formats
- Organization terminology overrides
- Localized notification templates
- Localized UI labels

## Non-Responsibilities

The engine does not automatically translate user-entered notes, comments, descriptions, or uploaded documents.

User-generated content should remain as entered unless a future approved feature changes that behavior.

## Translation Strategy

Use translation keys instead of hardcoded display text.

Examples:

```text
platform.actions.save
platform.actions.cancel
platform.errors.permission_denied
religious.members.title
```

Client applications should render labels from translation keys and organization configuration.

## Terminology Overrides

Organizations may customize selected labels.

Examples:

```json
{
  "memberLabel": "Member",
  "leaderLabel": "Leader",
  "branchLabel": "Branch"
}
```

This allows the same platform to serve different organization types without changing code.

## Regional Configuration

Regional settings may include:

- region_code
- default_currency
- default_timezone
- default_language
- supported_languages
- date_format
- time_format

Provider selection rules for SMS and payments should reference regional configuration but remain implemented in their own engines.

## Suggested Entities

Candidate entities:

- languages
- translation_keys
- translations
- regional_settings
- organization_localization_settings
- terminology_overrides

## Events Published

- LanguageEnabled
- TranslationUpdated
- RegionalSettingsUpdated
- OrganizationTerminologyUpdated
- OrganizationLocaleUpdated

## Security Requirements

- Platform-level localization changes require platform permission.
- Organization terminology changes require organization permission.
- Important changes should be auditable.

## Tenant Isolation

Organization terminology and localization settings must be scoped to the organization.

One organization's terminology must not affect another organization.

## Testing Requirements

Tests must cover:

- Translation key lookup
- Fallback language behavior
- Organization terminology override
- Regional default settings
- Permission checks
- Tenant isolation

## Final Rule

Do not hardcode language, region, currency, timezone, or terminology into domain logic.
