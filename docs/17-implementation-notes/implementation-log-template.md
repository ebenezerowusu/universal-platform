# Implementation Log Template

## Purpose

Use this template for every implementation sprint or PR.

The implementation log records what was built, what was intentionally avoided, and how architecture boundaries were preserved.

## Sprint / PR

```text
Sprint:
Branch:
PR:
Date:
Author:
```

## Summary

Describe what was implemented in this PR.

## Documents Consulted

```text
CLAUDE.md
Relevant ADRs
Relevant API contract
Relevant readiness document
Relevant traceability catalogue
```

## Files Changed

```text
<list files>
```

## Migrations Added

```text
<list migrations or none>
```

## API Routes Added Or Changed

```text
<list routes or none>
```

## UI Screens Added Or Changed

```text
<list screens or none>
```

## Tests Added

```text
<list tests>
```

## Checks Run

```text
cargo fmt
cargo check
cargo test
flutter analyze placeholder
other checks
```

## Architecture Boundaries Followed

Confirm:

```text
[ ] Core remains domain-agnostic
[ ] API handlers stay thin
[ ] Business logic is placed in the correct service/module
[ ] Tenant isolation is preserved
[ ] Permission checks are enforced where applicable
[ ] Sensitive actions are auditable where applicable
[ ] Provider-specific logic stays behind adapters
[ ] Configuration is not hardcoded
```

## Out Of Scope Items Avoided

List what was intentionally not implemented.

```text
<items>
```

## Deviations From Plan

State any difference from the documented plan.

## Reason For Deviation

Explain why the deviation was necessary.

## Events Added

```text
<events or none>
```

## Audit Events Added

```text
<audit events or none>
```

## Documentation Updates Needed

```text
<docs to update or none>
```

## Known Limitations

- 

## Follow-Up Work

- 

## Next Recommended Step

```text
<next implementation step>
```

## Final Rule

Every implementation PR should leave a clear record of what changed, why it changed, what checks were run, and what remains.
