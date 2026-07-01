# Universal Platform

Universal Platform is a multi-tenant, modular, extensible SaaS platform designed to help organizations digitize, manage, and grow their operations through installable domains, reusable capability engines, and a stable platform core.

## Guiding Vision

> Build the platform once. Expand it forever.

The platform must not be limited to one industry, one country, one payment provider, one SMS provider, or one technology stack. The first major domain is the Religious Organisation domain, but the platform is designed to support future domains such as Commerce, POS, Finance, HR, Logistics, Education, and other business verticals.

## Current Phase

The project is currently in **Phase 0: Foundation Documentation**.

No production code should be written until the foundational architecture, engineering principles, module structure, database strategy, infrastructure plan, and AI coding-agent rules are documented.

## Repository Structure

```text
docs/
  00-vision/
  01-platform-constitution/
  02-architecture/
  03-domains/
  04-capability-engines/
  05-infrastructure/
  06-development-standards/
  07-database/
  08-security/
  09-ai-development-handbook/
  10-adr/
  11-ui-ux/
  12-api/
  13-module-sdk/
  14-roadmap/

backend/
frontend/
infrastructure/
scripts/
```

## Foundation Rule

Every developer, AI coding agent, and contributor must read the documentation in `docs/` before implementing any feature.

The Platform Core must remain domain-agnostic. Business logic belongs inside domains and modules. External providers must be accessed through capability engines and adapters, never directly from domain logic.
