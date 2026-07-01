# Finance Domain Specification

## Purpose

The Finance Domain provides financial management capabilities for organizations.

It is separate from the Payment Engine. The Payment Engine processes transactions. The Finance Domain manages financial records, budgets, funds, approvals, and reports.

## Scope

The Finance Domain may include:

- Chart of accounts
- Income records
- Expense records
- Budgeting
- Funds and projects
- Approvals
- Petty cash later
- Reconciliation later
- Financial reports

## Out of Scope

The Finance Domain must not directly integrate with payment providers.

Payment provider handling belongs to the Payment Engine.

The Finance Domain may consume verified payment events and convert them into financial records according to accounting rules.

## Main Modules

### Chart of Accounts

Features:

- Account categories
- Account codes
- Income accounts
- Expense accounts
- Asset/liability/equity accounts later

### Income and Expenses

Features:

- Record income
- Record expense
- Attach documents
- Categorize transactions
- Approval workflow where needed

### Budgets

Features:

- Create budget
- Assign budget to branch/department/project
- Track planned vs actual
- Budget period

### Funds and Projects

Features:

- Create fund
- Track income for fund
- Track expenses for fund
- Project progress reports

### Financial Reports

Possible reports:

- Income summary
- Expense summary
- Budget performance
- Fund balance
- Branch finance summary
- Exportable reports

## Suggested Entities

Candidate entities:

- finance_accounts
- finance_income_records
- finance_expense_records
- finance_budgets
- finance_budget_lines
- finance_funds
- finance_projects
- finance_attachments
- finance_approval_records

## Permission Examples

- finance.accounts.view
- finance.accounts.manage
- finance.income.view
- finance.income.record
- finance.expenses.view
- finance.expenses.record
- finance.budgets.view
- finance.budgets.manage
- finance.reports.view
- finance.reports.export

## Events Published

- FinanceIncomeRecorded
- FinanceExpenseRecorded
- FinanceBudgetCreated
- FinanceBudgetUpdated
- FinanceReportExported

## Events Consumed

- PaymentCompleted
- PaymentFailed
- FileUploaded
- WorkflowTaskCompleted

## Platform Engine Usage

Finance Domain must use:

- Payment Engine events for verified payments
- Storage Engine for attachments
- Workflow Engine for approvals
- Reporting Engine for reports
- Audit Engine for sensitive changes
- Permission Engine for access control
- Organization Engine for tenant context

## Implementation Priority

Finance should be introduced after core payment and reporting foundations exist.

Recommended order:

1. Chart of accounts
2. Income and expense records
3. Attachments
4. Basic reports
5. Budgeting
6. Funds/projects
7. Approval workflow
8. Reconciliation later

## Final Rule

Finance owns accounting and financial management. Payment Engine owns payment processing.
