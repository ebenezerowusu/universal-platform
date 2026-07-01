# Commerce Domain Specification

## Purpose

The Commerce Domain defines organization-facing catalog, listing, ordering, and customer purchase workflows for Universal Platform.

It is designed as a domain module that uses shared platform engines instead of owning platform-wide infrastructure concerns.

## Scope

The Commerce Domain may include:

- Business profile
- Catalog listings
- Categories
- Customer cart
- Customer orders
- Order status tracking
- Customer notifications
- Basic commerce reports
- Integration with POS catalog later

## Out of Scope

The Commerce Domain must not own these platform capabilities:

- Payment processing
- SMS delivery
- File storage
- Generic notifications
- Tenant management
- Permissions
- Accounting

Those concerns belong to shared engines or other domains.

## Relationship with POS

Commerce and POS are related but separate.

POS manages internal product records, stock, and in-person sales workflows.

Commerce presents selected items or services for customer ordering.

A future sync can allow POS catalog items to be published as Commerce listings.

## Main Modules

### Business Profile

Features:

- Public profile
- Contact details
- Branding images through Storage Engine
- Visibility settings

### Catalog Listings

Features:

- Create listing
- Edit listing
- Publish listing
- Add images
- Set price
- Assign category
- Set availability

### Cart

Features:

- Add item
- Update quantity
- Remove item
- Prepare checkout request

### Orders

Features:

- Create order
- Track order status
- Store order items
- Track payment reference
- Send customer notification

Possible order statuses:

- pending
- payment_pending
- paid
- processing
- ready
- completed
- cancelled

## Suggested Entities

Candidate entities:

- commerce_profiles
- commerce_categories
- commerce_listings
- commerce_listing_images
- commerce_carts
- commerce_cart_items
- commerce_orders
- commerce_order_items
- commerce_order_status_history

## Permission Examples

- commerce.profile.view
- commerce.profile.update
- commerce.listings.view
- commerce.listings.create
- commerce.listings.update
- commerce.listings.publish
- commerce.orders.view
- commerce.orders.manage
- commerce.reports.view

## Events Published

- CommerceListingCreated
- CommerceListingPublished
- CommerceOrderCreated
- CommerceOrderPaid
- CommerceOrderStatusChanged

## Events Consumed

- PaymentCompleted
- PaymentFailed
- NotificationDelivered
- PosStockChanged later

## Platform Engine Usage

Commerce Domain must use:

- Organization Engine for tenant context
- Permission Engine for access control
- Payment Engine for payments
- Storage Engine for listing images
- Notification Engine for order updates
- Reporting Engine for reports
- Audit Engine for sensitive changes

## Implementation Priority

Recommended order:

1. Business profile
2. Categories
3. Listings
4. Cart
5. Orders
6. Payment integration
7. Notifications
8. POS catalog sync later

## Final Rule

Commerce owns customer ordering workflows. It must not own payment processing, storage, notification delivery, or accounting.
