# Configuration Engine Specification

## Purpose

The Configuration Engine provides a consistent way to manage platform, organization, module, and feature configuration.

It prevents hardcoded behavior and keeps the platform adaptable across countries, organizations, providers, and domains.

## Principle

Configuration controls behavior that should vary.

Business rules that are stable and domain-specific should remain in domains. Environment and provider details should remain outside business logic.

## Scope

The Configuration Engine may support:

- platform settings
- organization settings
- module settings
- feature settings
- default values
- setting validation
- configuration inheritance
- configuration change audit

## Configuration Levels

### Platform Level

Examples:

- supported countries
- supported languages
- provider availability
- global feature flags
- module catalog defaults

### Organization Level

Examples:

- timezone
- currency
- default language
- branding
- terminology
- communication preferences

### Module Level

Examples:

- attendance rules
- SMS settings
- payment settings
- member form settings
- event settings

## Suggested Entities

Candidate entities:

- configuration_definitions
- platform_configuration_values
- organization_configuration_values
- module_configuration_values
- configuration_change_history

## Setting Definition Fields

A setting definition may include:

- key
- scope
- value_type
- default_value
- required
- validation_rules
- description
- module_key nullable
- is_sensitive
- is_public_to_client

## Sensitive Configuration

Sensitive configuration must be handled carefully.

Examples:

- provider keys
- signing secrets
- private integration credentials

Sensitive values should not be stored as normal plain configuration values.

## Client Exposure

Only configuration marked safe for clients should be returned to Flutter or public APIs.

Examples of client-safe configuration:

- organization display name
- theme settings
- enabled language list
- terminology labels

## Required Engines

The Configuration Engine should integrate with:

- Audit Engine for configuration changes
- Permission Engine for update access
- Module Registry for module settings
- Localization Engine for locale-aware settings

## Events Published

- ConfigurationUpdated
- OrganizationConfigurationUpdated
- ModuleConfigurationUpdated

## Testing Requirements

Tests should cover:

- reading default configuration
- overriding organization configuration
- validating setting value
- hiding sensitive configuration
- auditing configuration changes

## Final Rule

Configuration must be explicit, validated, and safe. Do not hide important behavior in scattered hardcoded values.
