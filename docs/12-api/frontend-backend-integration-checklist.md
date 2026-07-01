# Frontend Backend Integration Checklist

## Purpose

This document defines the checklist for connecting Flutter screens to backend APIs.

It helps ensure Flutter integration follows backend contracts and platform rules.

## API Contract Check

Before integrating a screen, confirm:

- API route exists or is clearly mocked
- request shape is documented
- response shape is documented
- error shape follows standard response contract
- required authentication is known
- required organization context is known
- module/feature/permission behavior is known

## Flutter API Client Check

Confirm:

- endpoint is added through the shared API client layer
- standard success parsing is used
- standard error parsing is used
- trace id is preserved where available
- screen does not perform raw HTTP calls directly

## State Check

Confirm:

- selected organization state is used
- session state is used
- loading state is handled
- error state is handled
- empty state is handled
- unavailable state is handled where module/feature is unavailable

## Security Check

Confirm:

- Flutter does not assume permission equals backend approval
- backend denial is handled safely
- sensitive fields are not shown in lists unless required
- session expiration redirects or prompts safely

## UX Check

Confirm:

- form validation is helpful
- backend validation errors are displayed safely
- submit buttons show progress
- duplicate submissions are prevented where practical
- success feedback is clear

## Test Check

Confirm tests where practical for:

- success response rendering
- error response rendering
- loading state
- empty state
- form validation
- unavailable module state

## Documentation Check

If the API behavior differs from the documented contract, update the API document before or with the Flutter integration.

## Final Rule

Flutter integration should follow documented API contracts and shared client foundations. Do not patch around backend behavior inside screens.
