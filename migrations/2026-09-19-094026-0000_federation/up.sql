-- Your SQL goes here

CREATE TABLE identity_providers (
  id UUID PRIMARY KEY,
  key TEXT NOT NULL,
  display_name TEXT NOT NULL,
  protocol TEXT NOT NULL CHECK (protocol IN ('oidc', 'oauth2')),
  issuer TEXT NULL,
  client_id TEXT NOT NULL,
  client_secret_ref TEXT NULL,
  discovery_url TEXT NULL,
  authorization_url TEXT NULL,
  token_url TEXT NULL,
  userinfo_url TEXT NULL,
  jwks_url TEXT NULL,
  status TEXT NOT NULL CHECK (status IN ('active', 'disabled')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL
);

CREATE UNIQUE INDEX uq_identity_providers_key
  ON identity_providers (key)
  WHERE deleted_at IS NULL;

CREATE TABLE federated_identities (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  provider_id UUID NOT NULL REFERENCES identity_providers(id) ON DELETE RESTRICT,
  provider_subject TEXT NOT NULL,
  last_used_at TIMESTAMPTZ NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL
);

CREATE UNIQUE INDEX uq_federated_identities_provider_subject
  ON federated_identities (provider_id, provider_subject)
  WHERE deleted_at IS NULL;

CREATE INDEX idx_federated_identities_user_id
  ON federated_identities (user_id)
  WHERE deleted_at IS NULL;
