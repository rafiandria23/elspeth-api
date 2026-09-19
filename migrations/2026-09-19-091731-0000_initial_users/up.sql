-- Your SQL goes here

CREATE TABLE users (
  id UUID PRIMARY KEY,
  status TEXT NOT NULL CHECK (status IN ('active', 'suspended', 'deactivated')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL
);

CREATE TABLE sign_in_identifiers (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  type TEXT NOT NULL CHECK (type IN ('email', 'username')),
  identifier TEXT NOT NULL,
  normalized_identifier TEXT NOT NULL,
  is_primary BOOLEAN NOT NULL DEFAULT FALSE,
  verified_at TIMESTAMPTZ NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL
);

-- Partial unique indexes for soft-deletion support

CREATE UNIQUE INDEX uq_sign_in_identifiers_type_normalized
  ON sign_in_identifiers (type, normalized_identifier)
  WHERE deleted_at IS NULL;

CREATE UNIQUE INDEX uq_sign_in_identifiers_user_primary_type
  ON sign_in_identifiers (user_id, type)
  WHERE is_primary = TRUE AND deleted_at IS NULL;

-- Query optimization index for user lookups
CREATE INDEX idx_sign_in_identifiers_user_id
  ON sign_in_identifiers (user_id)
  WHERE deleted_at IS NULL;
