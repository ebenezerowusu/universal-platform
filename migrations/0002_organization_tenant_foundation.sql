-- Sprint 3: Organization/Tenant foundation
-- Purpose: create the generic platform organization and membership tables.
--
-- Organizations are tenant containers. They are not churches, shops, schools,
-- partners, or domain-specific profiles. Domains attach their own data to the
-- organization tenant boundary.

CREATE TABLE organizations (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    organization_type TEXT NOT NULL DEFAULT 'generic',
    status TEXT NOT NULL DEFAULT 'pending_setup',
    country_code CHAR(2) NOT NULL,
    currency_code CHAR(3) NOT NULL,
    timezone TEXT NOT NULL,
    default_language TEXT NOT NULL DEFAULT 'en',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    suspended_at TIMESTAMPTZ NULL,
    deactivated_at TIMESTAMPTZ NULL,
    CONSTRAINT organizations_name_not_blank CHECK (length(trim(name)) >= 2),
    CONSTRAINT organizations_slug_not_blank CHECK (length(trim(slug)) >= 3),
    CONSTRAINT organizations_type_check CHECK (
        organization_type IN (
            'generic',
            'religious',
            'commerce',
            'non_profit',
            'education',
            'other'
        )
    ),
    CONSTRAINT organizations_status_check CHECK (
        status IN (
            'pending_setup',
            'active',
            'suspended',
            'deactivated'
        )
    ),
    CONSTRAINT organizations_country_code_check CHECK (country_code ~ '^[A-Z]{2}$'),
    CONSTRAINT organizations_currency_code_check CHECK (currency_code ~ '^[A-Z]{3}$')
);

CREATE UNIQUE INDEX organizations_slug_unique_idx
    ON organizations (lower(slug));

CREATE INDEX organizations_status_idx
    ON organizations (status);

CREATE INDEX organizations_country_code_idx
    ON organizations (country_code);

CREATE INDEX organizations_created_at_idx
    ON organizations (created_at);

CREATE TABLE organization_memberships (
    id UUID PRIMARY KEY,
    organization_id UUID NOT NULL REFERENCES organizations(id),
    user_id UUID NOT NULL REFERENCES identity_users(id),
    status TEXT NOT NULL DEFAULT 'active',
    joined_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    suspended_at TIMESTAMPTZ NULL,
    revoked_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT organization_memberships_status_check CHECK (
        status IN (
            'invited',
            'active',
            'suspended',
            'revoked'
        )
    )
);

CREATE UNIQUE INDEX organization_memberships_unique_active_user_org_idx
    ON organization_memberships (organization_id, user_id)
    WHERE status IN ('invited', 'active', 'suspended');

CREATE INDEX organization_memberships_organization_id_idx
    ON organization_memberships (organization_id);

CREATE INDEX organization_memberships_user_id_idx
    ON organization_memberships (user_id);

CREATE INDEX organization_memberships_status_idx
    ON organization_memberships (status);

CREATE INDEX organization_memberships_joined_at_idx
    ON organization_memberships (joined_at);
