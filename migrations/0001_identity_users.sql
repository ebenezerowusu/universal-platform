-- Sprint 2: Identity foundation
-- Purpose: create the global identity user table.
--
-- Identity users are platform-level accounts. Organization membership and
-- tenant-scoped access will be introduced in the Organization/Tenant foundation.

CREATE TABLE identity_users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending_verification',
    email_verified_at TIMESTAMPTZ NULL,
    last_login_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deactivated_at TIMESTAMPTZ NULL,
    CONSTRAINT identity_users_email_not_blank CHECK (length(trim(email)) > 0),
    CONSTRAINT identity_users_status_check CHECK (
        status IN (
            'pending_verification',
            'active',
            'suspended',
            'deactivated'
        )
    )
);

CREATE UNIQUE INDEX identity_users_email_unique_idx
    ON identity_users (lower(email));

CREATE INDEX identity_users_status_idx
    ON identity_users (status);

CREATE INDEX identity_users_created_at_idx
    ON identity_users (created_at);
