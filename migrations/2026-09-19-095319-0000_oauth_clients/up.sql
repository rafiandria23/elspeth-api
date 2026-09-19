-- Your SQL goes here

CREATE TABLE oauth_clients (
  id UUID PRIMARY KEY,
  client_id TEXT NOT NULL,
  client_secret_hash TEXT NULL,
  name TEXT NOT NULL,
  client_type TEXT NOT NULL CHECK (client_type IN ('confidential', 'public')),
  is_first_party BOOLEAN NOT NULL DEFAULT FALSE,
  status TEXT NOT NULL CHECK (status IN ('active', 'suspended')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL
);

CREATE UNIQUE INDEX uq_oauth_clients_client_id
  ON oauth_clients (client_id)
  WHERE deleted_at IS NULL;

CREATE TABLE oauth_client_redirect_uris (
  id UUID PRIMARY KEY,
  client_id UUID NOT NULL REFERENCES oauth_clients(id) ON DELETE CASCADE,
  redirect_uri TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL
);

CREATE UNIQUE INDEX uq_oauth_client_redirect_uris_client_redirect
  ON oauth_client_redirect_uris (client_id, redirect_uri)
  WHERE deleted_at IS NULL;

CREATE INDEX idx_oauth_client_redirect_uris_client_id
  ON oauth_client_redirect_uris (client_id)
  WHERE deleted_at IS NULL;

CREATE TABLE oauth_scopes (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  is_system BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL
);

CREATE UNIQUE INDEX uq_oauth_scopes_name
  ON oauth_scopes (name)
  WHERE deleted_at IS NULL;

CREATE TABLE oauth_client_scopes (
  client_id UUID NOT NULL REFERENCES oauth_clients(id) ON DELETE CASCADE,
  scope_id UUID NOT NULL REFERENCES oauth_scopes(id) ON DELETE CASCADE,
  is_default BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL,
  PRIMARY KEY (client_id, scope_id)
);

CREATE INDEX idx_oauth_client_scopes_scope_id
  ON oauth_client_scopes (scope_id);
