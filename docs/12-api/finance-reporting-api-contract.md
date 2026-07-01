# Finance Reporting API Contract

## Purpose

This document defines the MVP API contract for finance reporting and cash-position visibility.

The API should summarize organization-owned income and expense records while preserving permission checks and data-quality warnings.

## Owner

```text
Finance Domain
Reporting Engine where practical
Import/Export Engine where practical
```

## Base Path

```text
/api/v1/organizations/{organizationId}/finance/reports
```

## Common Query Parameters

```text
period
startDate
endDate
currency
categoryId
```

## Dashboard Summary

```text
GET /api/v1/organizations/{organizationId}/finance/reports/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "summary": {
      "totalIncome": 12000,
      "totalExpenses": 4500,
      "netPosition": 7500,
      "pendingIncome": 500,
      "pendingExpenses": 800,
      "unreconciledItems": 3,
      "currency": "GHS"
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Income Summary

```text
GET /api/v1/organizations/{organizationId}/finance/reports/income-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "categoryId": "...",
        "categoryName": "General Giving",
        "amount": 5000,
        "currency": "GHS"
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Expense Summary

```text
GET /api/v1/organizations/{organizationId}/finance/reports/expense-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "categoryId": "...",
        "categoryName": "Operations",
        "amount": 2500,
        "currency": "GHS"
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Net Position

```text
GET /api/v1/organizations/{organizationId}/finance/reports/net-position
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "netPosition": {
      "income": 12000,
      "expenses": 4500,
      "net": 7500,
      "currency": "GHS"
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Budget Comparison

```text
GET /api/v1/organizations/{organizationId}/finance/reports/budget-comparison
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "budgetLineId": "...",
        "categoryName": "Operations",
        "budgetedAmount": 1000,
        "recordedExpenses": 650,
        "remainingAmount": 350,
        "currency": "GHS"
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Data Quality Warnings

```text
GET /api/v1/organizations/{organizationId}/finance/reports/data-quality-warnings
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "warnings": [
      {
        "type": "unreconciled_income",
        "count": 2,
        "severity": "medium"
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Export Request

```text
POST /api/v1/organizations/{organizationId}/finance/reports/export-requests
```

### Request Direction

```json
{
  "reportType": "income_summary",
  "format": "csv",
  "filters": {
    "period": "this_month"
  }
}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- Finance module availability
- required permission
- feature availability
- organization ownership

Export routes should additionally check export permission.

## Audit Direction

Audit should be written for:

- export requested
- export downloaded where implemented
- sensitive finance report viewed where policy requires it

## Error Direction

Use standard response shape.

Expected error areas:

- report unavailable
- invalid period
- invalid date range
- permission denied
- export not allowed
- organization not found

## Final Rule

Finance reporting APIs must aggregate only organization-owned records and must expose warnings when data quality affects report confidence.
