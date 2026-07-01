# Design System Component Catalog

## Purpose

This document defines the first shared UI component catalog for Universal Platform.

The same design system should support mobile, desktop/admin, and future web surfaces.

## Design Principle

Build reusable platform components before building one-off screens.

The UI should be module-aware, permission-aware, responsive, and localization-ready.

## Core Components

Initial components:

- App shell
- Navigation sidebar
- Bottom navigation
- Top bar
- Page header
- Section header
- Cards
- Buttons
- Text fields
- Dropdowns
- Date pickers
- File upload controls
- Empty states
- Loading states
- Error states
- Confirmation dialogs
- Toast/snackbar messages

## Data Components

Desktop/admin data components:

- Data table
- Filters panel
- Search input
- Pagination controls
- Bulk action bar
- Detail drawer
- Status badge
- Export action button

Mobile data components:

- List item
- Profile card
- Action tile
- Quick action button
- Pull-to-refresh list
- Compact filter sheet

## Domain Components

Religious Domain component candidates:

- Member card
- Visitor card
- Attendance status selector
- Group selector
- Branch selector
- Event card
- Giving summary card
- Follow-up task card

Commerce/POS future component candidates:

- Product card
- Cart item
- Order status badge
- POS product tile
- Stock badge

## Feedback Components

The platform should standardize:

- success messages
- validation errors
- permission denied states
- empty module states
- upgrade prompts
- loading skeletons

## Accessibility Direction

Components should consider:

- readable text sizes
- proper contrast
- keyboard navigation for desktop
- screen reader labels where practical
- tap target sizes for mobile

## Localization Direction

Components should use label keys and translated text.

Avoid hardcoded strings in reusable components.

## Final Rule

Reusable components protect consistency. Do not build every screen from scratch.
