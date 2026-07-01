# AI Coding Agent Rules

## Purpose

This document defines how AI coding agents such as Claude Code, ChatGPT, Codex, Cursor, Gemini, and Windsurf must work on Universal Platform.

The platform is architecture-first. AI agents must not treat prompts as isolated coding tasks.

## Required Reading Before Coding

Before implementing anything, the AI agent must read:

1. `docs/00-vision/00-platform-vision.md`
2. `docs/01-platform-constitution/01-platform-constitution.md`
3. `docs/02-architecture/02-platform-architecture.md`
4. `docs/06-development-standards/06-engineering-standards.md`
5. `docs/07-database/07-database-strategy.md`
6. The relevant domain or engine document for the task
7. Relevant ADRs

If these files are not present or not read, the agent must not start implementation.

## Prime Directive

> Do not violate the Platform Constitution.

If the user asks for something that conflicts with the Platform Constitution, the agent must stop, explain the conflict, and suggest a compliant implementation.

## Classification Before Implementation

Before coding any feature, the agent must classify it as one of:

- Platform Core
- Capability Engine
- Domain Feature
- Module Feature
- Infrastructure Adapter
- Client UI Feature
- Developer Tooling

The agent must explain the classification briefly in its implementation plan.

## Architecture Questions

For every feature, answer:

1. Does this belong in Platform Core or a domain?
2. Can it become a reusable capability engine?
3. Does it require tenant isolation?
4. Does it require permissions?
5. Does it need an event?
6. Does it call an external provider?
7. Is provider access hidden behind an adapter?
8. Is behavior configurable?
9. Is the database design relationally sound?
10. What tests prove it works safely?

## Forbidden AI Behaviors

AI agents must not:

- Put business logic in controllers/handlers.
- Put business logic in Flutter UI.
- Call SMS/payment/storage providers directly from domain modules.
- Hardcode Ghana, Arkessel, Hubtel, church, pastor, member, or similar domain/provider/country concepts into Platform Core.
- Skip tenant filters in queries.
- Create database tables without considering organization ownership.
- Create features without permissions.
- Create provider-specific code inside domains.
- Create undocumented architecture decisions.
- Ignore existing documentation.
- Generate large code without explaining the module boundaries first.

## Required Implementation Pattern

A good implementation plan should include:

```text
1. Classification
2. Files to create/change
3. Domain/application/infrastructure boundaries
4. Database/migration impact
5. Permissions required
6. Events published/consumed
7. Tests to add
8. Documentation updates
```

## Code Quality Expectations

AI-generated code must be:

- Clear
- Typed
- Tested
- Modular
- Tenant-aware
- Permission-aware
- Error-safe
- Provider-agnostic
- Consistent with existing naming

## Database Rules for AI Agents

When creating database changes:

- Use migrations.
- Include organization_id for tenant-owned records.
- Add indexes for common queries.
- Add foreign keys where appropriate.
- Avoid JSONB for core relational data.
- Use JSONB only for flexible metadata/settings/custom fields.
- Include auditability where needed.

## Provider Integration Rules

For external integrations:

```text
Domain -> Capability Engine -> Provider Interface -> Provider Adapter
```

Never:

```text
Domain -> Provider SDK/API
```

Example:

Religious Giving must call Payment Engine. It must not call Hubtel directly.

Birthday SMS must call SMS Engine. It must not call Arkessel directly.

## Event Rules

When a business action occurs, consider publishing an event.

Examples:

- UserCreated
- OrganizationCreated
- ModuleInstalled
- PaymentCompleted
- SmsSent
- AttendanceMarked
- MemberRegistered
- VisitorConverted

Do not use direct module calls when events are more appropriate.

## Output Rule

When the AI agent completes a task, it should summarize:

- What changed
- Why it changed
- Files changed
- Tests run
- Architecture rules followed
- Any risks or follow-up work

## Final Rule

The AI agent is not just a code generator. It is a guardian of the platform architecture.
