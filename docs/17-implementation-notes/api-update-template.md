# API Update Template

## Purpose

Use this template whenever routes are implemented, changed, deprecated, or removed.

API update notes keep implemented routes aligned with `docs/18-traceability/api-route-catalogue.md` and the relevant API contract.

## Sprint / PR

```text
Sprint:
Branch:
PR:
Date:
Author:
```

## Related API Contract

```text
<path to docs/12-api/...>
```

## Routes Added

| Method | Route | Purpose | Permission |
|---|---|---|---|
| GET | `/api/v1/...` | `<purpose>` | `<permission>` |

## Routes Changed

| Method | Route | Change | Reason |
|---|---|---|---|
| PATCH | `/api/v1/...` | `<change>` | `<reason>` |

## Routes Deprecated Or Removed

| Method | Route | Status | Replacement |
|---|---|---|---|
| GET | `/api/v1/...` | deprecated/removed | `<replacement>` |

## Request/Response Summary

Describe major request and response shapes.

```json
{
  "success": true,
  "data": {},
  "meta": {
    "traceId": "..."
  }
}
```

## Authentication And Authorization

Confirm:

```text
[ ] Authentication required where applicable
[ ] organization_id or tenant scope enforced where applicable
[ ] Permission checks enforced
[ ] Feature/module entitlement checks enforced where applicable
[ ] Audit records created for sensitive actions
```

## Error Cases

List expected errors:

```text
401 unauthenticated
403 permission denied
404 not found
409 conflict
422 validation error
500 internal error
```

## Events And Audit

Domain events emitted:

```text
<events or none>
```

Audit events written:

```text
<audit events or none>
```

## Tests Added

```text
<test files or test cases>
```

## Documentation Updated

Confirm updates:

```text
[ ] API contract updated if needed
[ ] API route catalogue updated if needed
[ ] Permission catalogue updated if needed
[ ] Event catalogue updated if needed
[ ] Audit event catalogue updated if needed
```

## Known Limitations

- 

## Final Rule

Implemented routes must be tenant-safe, permission-protected, validated, auditable where sensitive, and documented.
