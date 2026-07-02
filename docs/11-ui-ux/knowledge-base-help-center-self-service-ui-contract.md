# Knowledge Base Help Center Self Service UI Contract

## Purpose

This document defines the MVP UI contract for Knowledge Base, Help Center, and Self-Service Support foundation.

The screens should help permitted users find help articles, browse categories, review article details, submit feedback, escalate unresolved issues, and manage help content where authorized.

## Navigation Direction

Knowledge base screens should appear when:

- user is authenticated where required
- organization is selected where required
- knowledge base feature is available
- user has required permission
- article visibility rules allow access

## Help Center Dashboard

Should show:

- published article count
- draft article count
- open feedback count
- support escalations from articles
- outdated article count
- popular categories placeholder

## Article List

Should show:

- article title
- category
- owner module/domain
- status
- updated date placeholder
- visibility safe summary
- action to view detail

## Article Detail

Should show:

- title
- article body/content reference placeholder
- category/collection
- related articles placeholder
- related policy/training links where available
- feedback action
- support escalation action

## Category and Collection List

Should show:

- category/collection title
- description safe summary
- article count placeholder
- visibility safe summary
- status

## Article Version Detail

Should show:

- version label
- content reference safe label
- document/media reference safe label where available
- published date placeholder
- replaced version placeholder
- status

## Search Results Placeholder

Should show:

- query safe summary
- matched article safe labels
- category filters
- module filters placeholder
- no-result support escalation action

## Feedback Screen

Should show:

- useful/not useful options
- issue still unresolved placeholder
- suggested improvement placeholder
- content outdated placeholder
- submission status

## Support Escalation Placeholder

Should show:

- article safe label
- user issue summary
- support case link placeholder
- escalation status
- next step placeholder

## Related Content Links

Should show:

- related policy safe label
- related training safe label
- related document safe label
- related support case safe label where available
- permission/masking indicators

## Help Content Audit History

Should show:

- event type
- actor safe label
- target safe label
- timestamp
- safe metadata summary

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied
- no search results

## MVP Exclusions

Do not implement in Sprint 58:

- full CMS editor UI
- AI support bot UI as source of truth
- unrestricted public publishing UI
- document permission bypass UI
- cross-tenant private article browser UI
- arbitrary script widget UI
- replacement support desk UI

## Final Rule

Knowledge base UI should help users resolve problems quickly while making escalation clear when self-service is not enough.
