# Country Localization Regional Operations API Contract

## Purpose

This document defines the MVP API contract for Country, Localization, and Regional Operations foundation.

The API should support country catalog metadata, currencies, timezones, languages, organization regional settings, formatting metadata, provider availability mappings, translation namespace placeholders, and regional readiness checks while preserving organization boundaries.

## Owner

```text
Regional Operations Foundation
Localization Engine
Payment Engine where provider availability is used
SMS Engine where provider availability is used
Permission Engine
Audit Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/regional-settings
```

## Regional Dashboard

```text
GET /api/v1/organizations/{organizationId}/regional-settings/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "countryCode": "GH",
    "currencyCode": "GHS",
    "timezone": "Africa/Accra",
    "defaultLanguage": "en",
    "regionalReadinessStatus": "ready_placeholder"
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Organization Regional Settings

```text
GET /api/v1/organizations/{organizationId}/regional-settings/current
PATCH /api/v1/organizations/{organizationId}/regional-settings/current
```

### Update Regional Settings Request

```json
{
  "countryCode": "GH",
  "currencyCode": "GHS",
  "timezone": "Africa/Accra",
  "defaultLanguage": "en",
  "supportedLanguages": ["en", "tw"],
  "dateFormat": "DD/MM/YYYY",
  "numberFormat": "standard_placeholder",
  "currencyFormat": "symbol_before_placeholder"
}
```

## Country Catalog

```text
GET /api/v1/organizations/{organizationId}/regional-settings/countries
POST /api/v1/organizations/{organizationId}/regional-settings/countries
GET /api/v1/organizations/{organizationId}/regional-settings/countries/{countryCode}
PATCH /api/v1/organizations/{organizationId}/regional-settings/countries/{countryCode}
```

Catalog create/update should be platform-admin or regional-admin only where applicable.

## Currency Catalog

```text
GET /api/v1/organizations/{organizationId}/regional-settings/currencies
POST /api/v1/organizations/{organizationId}/regional-settings/currencies
GET /api/v1/organizations/{organizationId}/regional-settings/currencies/{currencyCode}
PATCH /api/v1/organizations/{organizationId}/regional-settings/currencies/{currencyCode}
```

## Timezone Catalog

```text
GET /api/v1/organizations/{organizationId}/regional-settings/timezones
POST /api/v1/organizations/{organizationId}/regional-settings/timezones
GET /api/v1/organizations/{organizationId}/regional-settings/timezones/{timezoneKey}
PATCH /api/v1/organizations/{organizationId}/regional-settings/timezones/{timezoneKey}
```

## Language Catalog

```text
GET /api/v1/organizations/{organizationId}/regional-settings/languages
POST /api/v1/organizations/{organizationId}/regional-settings/languages
GET /api/v1/organizations/{organizationId}/regional-settings/languages/{languageCode}
PATCH /api/v1/organizations/{organizationId}/regional-settings/languages/{languageCode}
```

## Provider Availability Mapping

```text
GET /api/v1/organizations/{organizationId}/regional-settings/provider-availability
POST /api/v1/organizations/{organizationId}/regional-settings/provider-availability
PATCH /api/v1/organizations/{organizationId}/regional-settings/provider-availability/{mappingId}
```

### Create Provider Availability Mapping

```json
{
  "providerType": "payment",
  "providerKey": "hubtel_placeholder",
  "countryCode": "GH",
  "currencyCode": "GHS",
  "methodType": "mobile_money_placeholder",
  "status": "active"
}
```

## Translation Namespace Placeholder

```text
GET /api/v1/organizations/{organizationId}/regional-settings/translation-namespaces
POST /api/v1/organizations/{organizationId}/regional-settings/translation-namespaces
PATCH /api/v1/organizations/{organizationId}/regional-settings/translation-namespaces/{namespaceId}
```

## Regional Readiness Checklist

```text
GET /api/v1/organizations/{organizationId}/regional-settings/readiness-checklist
PATCH /api/v1/organizations/{organizationId}/regional-settings/readiness-checklist/{itemId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership or permitted platform regional role
- regional settings feature availability
- required permission
- organization ownership
- provider availability compatibility where practical

Regional settings must not cross organization boundaries.

## Status Direction

Suggested catalog statuses:

```text
active
inactive
planned_placeholder
manual_review
```

Suggested readiness statuses:

```text
not_started
in_progress
ready_placeholder
blocked
manual_review
```

Suggested provider availability statuses:

```text
active
inactive
planned_placeholder
unsupported
manual_review
```

## Audit Direction

Audit should be written for:

- organization country changed
- organization currency changed
- organization timezone changed
- organization language settings changed
- provider availability changed
- catalog metadata changed
- translation namespace changed
- readiness checklist updated

## Error Direction

Use standard response shape.

Expected error areas:

- country not found
- currency not found
- timezone not found
- language not found
- provider availability not found
- unsupported provider/currency/country combination
- regional settings permission denied
- organization not found

## Final Rule

Regional settings APIs must be configuration-driven, organization-scoped, permission-protected, and free from country-specific business logic in Core.
