# POS Domain Specification

## Purpose

The POS Domain provides business-facing point-of-sale and inventory capabilities for organizations that sell goods or services.

The POS Domain is separate from the Commerce Domain, but the two can integrate through catalog and order events.

## Scope

The POS Domain may include:

- Product catalog
- Inventory
- Stock movement
- Sales records
- Cashier sessions
- Receipts
- Store/location support
- Barcode support later
- Daily sales reports

## Out of Scope

The POS Domain must not implement generic platform services directly.

- Payment provider integration belongs to Payment Engine.
- File/image handling belongs to Storage Engine.
- Reports use Reporting Engine patterns.
- Accounting belongs to Finance Domain.

## Main Modules

### Product Catalog

Features:

- Create products
- Product categories
- SKU/barcode
- Product image
- Unit price
- Cost price where allowed
- Active/inactive status

### Inventory

Features:

- Stock quantity
- Stock adjustment
- Stock transfer later
- Low stock alerts
- Inventory history

### Sales

Features:

- Create sale
- Add sale items
- Calculate totals
- Record payment method reference
- Generate receipt
- Void/reversal policy later

### Cashier Sessions

Features:

- Open session
- Record sales
- Close session
- Daily summary

## Suggested Entities

Candidate entities:

- pos_products
- pos_product_categories
- pos_inventory_items
- pos_stock_movements
- pos_sales
- pos_sale_items
- pos_cashier_sessions
- pos_receipts

## Permission Examples

- pos.products.view
- pos.products.create
- pos.products.update
- pos.inventory.view
- pos.inventory.adjust
- pos.sales.create
- pos.sales.view
- pos.reports.view

## Events Published

- PosProductCreated
- PosStockAdjusted
- PosSaleCreated
- PosSaleCompleted
- PosLowStockDetected

## Platform Engine Usage

POS Domain must use:

- Storage Engine for product images
- Payment Engine for digital payment flows where needed
- Reporting Engine for sales reports
- Notification Engine for low-stock alerts
- Permission Engine for protected actions
- Audit Engine for stock and sales changes

## Implementation Priority

Recommended order:

1. Product categories
2. Products
3. Inventory quantities
4. Stock movements
5. Basic sale records
6. Receipts
7. Reports
8. Commerce listing sync later

## Final Rule

POS owns in-store sales and inventory workflows. It must not own platform payment processing or accounting.
