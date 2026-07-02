# Country Localization Regional Operations Guide

## Purpose

This document defines operating guidance for country metadata, currencies, timezones, languages, organization regional settings, formatting, provider availability, and translation namespace placeholders.

The goal is to make regional behavior configurable and safe across all domains.

## Principle

Country behavior should be selected through configuration, not hardcoded into Core or modules.

## Data Sources

Regional operations may use:

- country catalog metadata
- currency metadata
- timezone metadata
- language catalog metadata
- organization regional settings
- format metadata
- payment provider availability mapping
- SMS provider availability mapping
- translation key namespace placeholder
- regional readiness checklist item
- audit records

## Country Direction

Country catalog metadata should capture:

```text
country_code
country_name
region_placeholder
default_currency
default_timezone
default_language
status
```

## Currency Direction

Currency metadata should capture:

```text
currency_code
symbol_placeholder
decimal_precision
supported_countries_placeholder
status
```

## Timezone Direction

Timezone metadata should support organization scheduling, reporting periods, notifications, attendance sessions, and calendar behavior.

## Language Direction

Language metadata should capture:

```text
language_code
display_name
direction_placeholder
status
default_flag_placeholder
```

Do not auto-translate user-generated content by default.

## Organization Regional Settings Direction

Organization regional settings should capture:

```text
organization_id
country_code
currency_code
timezone
default_language
supported_languages
date_format
number_format_placeholder
currency_format_placeholder
```

## Provider Availability Direction

Provider availability mappings should capture:

```text
provider_type
provider_key
country_code
currency_code optional
method_type optional
status
```

Payment Engine and SMS Engine should read these mappings before presenting or using providers.

## Formatting Direction

Formatting should support:

- date format
- time format placeholder
- number format placeholder
- currency format placeholder
- language fallback placeholder

## Translation Namespace Direction

Translation namespaces may include:

- platform labels
- module labels
- notification labels
- report labels
- validation messages
- workflow labels

## Audit Direction

Audit should record:

- organization country changed
- organization currency changed
- organization timezone changed
- language settings changed
- provider availability changed
- catalog metadata changed
- translation namespace changed
- readiness checklist updated

## Data Quality Warnings

Warn or flag when:

- organization has no country selected
- selected currency is not supported in selected country
- selected provider is unavailable for country/currency
- timezone is missing
- default language is not in supported languages
- provider availability is incomplete for payment or SMS

## Final Rule

Regional operations must keep country, currency, language, timezone, and provider behavior aligned without hardcoding country rules into Core.
