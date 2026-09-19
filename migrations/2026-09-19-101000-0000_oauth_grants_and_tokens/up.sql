-- Your SQL goes here

CREATE TABLE oauth_grants (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  client_id UUID NOT NULL REFERENCES oauth_clients(id) ON DELETE CASCADE,
  status TEXT NOT NULL CHECK (status IN ('active', 'revoked')),
  revoked_at TIMESTAMPTZ NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL
);

CREATE UNIQUE INDEX uq_oauth_grants_user_client
  ON oauth_grants (user_id, client_id)
  WHERE deleted_at IS NULL;

CREATE INDEX idx_oauth_grants_user_id
  ON oauth_grants (user_id)
  WHERE deleted_at IS NULL;

CREATE INDEX idx_oauth_grants_client_id
  ON oauth_grants (client_id)
  WHERE deleted_at IS NULL;

CREATE TABLE oauth_grant_scopes (
  grant_id UUID NOT NULL REFERENCES oauth_grants(id) ON DELETE CASCADE,
  scope_id UUID NOT NULL REFERENCES oauth_scopes(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL,
  PRIMARY KEY (grant_id, scope_id)
);

CREATE INDEX idx_oauth_grant_scopes_scope_id
  ON oauth_grant_scopes (scope_id);

CREATE TABLE oauth_refresh_tokens (
  id UUID PRIMARY KEY,
  family_id UUID NOT NULL,
  token_hash BYTEA NOT NULL,
  grant_id UUID NOT NULL REFERENCES oauth_grants(id) ON DELETE CASCADE,
  status TEXT NOT NULL CHECK (status IN ('active', 'consumed', 'revoked')),
  granted_scopes TEXT[] NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL,
  consumed_at TIMESTAMPTZ NULL,
  replaced_by_id UUID NULL REFERENCES oauth_refresh_tokens(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL
);

CREATE UNIQUE INDEX uq_oauth_refresh_tokens_token_hash
  ON oauth_refresh_tokens (token_hash)
  WHERE deleted_at IS NULL;

CREATE INDEX idx_oauth_refresh_tokens_family_id
  ON oauth_refresh_tokens (family_id);

CREATE INDEX idx_oauth_refresh_tokens_grant_id
  ON oauth_refresh_tokens (grant_id);
