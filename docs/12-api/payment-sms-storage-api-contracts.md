# Payment, SMS, and Storage API Contracts

## Purpose

This document defines planning-level API contracts for Payment Engine, SMS Engine, and Storage Engine.

These engines provide shared platform capabilities and must not be implemented directly inside product domains.

## Payment: Initialize Transaction

```text
POST /api/v1/organizations/{organizationId}/payments/transactions
```

Request example:

```json
{
  "purpose": "religious_giving",
  "amount": 100.0,
  "currency": "GHS",
  "referenceOwnerType": "religious_giving_record",
  "referenceOwnerId": "...",
  "paymentMethod": "mobile_money",
  "metadata": {}
}
```

Required permission depends on the calling use case.

Response data:

```json
{
  "transaction": {
    "id": "...",
    "status": "initialized",
    "amount": 100.0,
    "currency": "GHS"
  },
  "checkout": {
    "type": "redirect_or_inline",
    "url": "..."
  }
}
```

## Payment: Verify Transaction

```text
POST /api/v1/organizations/{organizationId}/payments/transactions/{transactionId}/verify
```

Response data:

```json
{
  "transaction": {
    "id": "...",
    "status": "successful"
  }
}
```

Events may include:

- PaymentCompleted
- PaymentFailed

## Payment: List Transactions

```text
GET /api/v1/organizations/{organizationId}/payments/transactions
```

Required permission:

```text
payments.transactions.view
```

## SMS: Wallet

```text
GET /api/v1/organizations/{organizationId}/sms/wallet
```

Response data:

```json
{
  "wallet": {
    "balance": 1000,
    "totalPurchased": 2000,
    "totalUsed": 1000
  }
}
```

Required permission:

```text
sms.wallet.view
```

## SMS: Send Message

```text
POST /api/v1/organizations/{organizationId}/sms/messages
```

Request example:

```json
{
  "recipients": ["0240000000"],
  "message": "Hello",
  "campaignId": "..."
}
```

Required permission:

```text
sms.messages.send
```

The SMS Engine should check wallet balance and provider availability.

## SMS: Message History

```text
GET /api/v1/organizations/{organizationId}/sms/messages
```

Required permission:

```text
sms.messages.view
```

## Storage: Request Upload

```text
POST /api/v1/organizations/{organizationId}/storage/files
```

Request example:

```json
{
  "ownerType": "religious_member",
  "ownerId": "...",
  "fileName": "profile.jpg",
  "mimeType": "image/jpeg",
  "visibility": "private"
}
```

Response data:

```json
{
  "file": {
    "id": "...",
    "status": "pending_upload"
  },
  "upload": {
    "type": "direct_or_backend",
    "url": "..."
  }
}
```

## Storage: File Metadata

```text
GET /api/v1/organizations/{organizationId}/storage/files/{fileId}
```

## Storage: Temporary Access

```text
POST /api/v1/organizations/{organizationId}/storage/files/{fileId}/temporary-access
```

This route must enforce organization context, permission, and file visibility rules.

## Final Rule

Domains request payment, SMS, and file capabilities through engines. Provider details remain hidden behind adapters.
