-- Your SQL goes here

CREATE TABLE signing_keys (
  id UUID PRIMARY KEY,
  kid TEXT NOT NULL,
  use_purpose TEXT NOT NULL CHECK (use_purpose IN ('sig', 'enc')),
  algorithm TEXT NOT NULL,
  public_jwk JSONB NOT NULL,
  private_key_ref TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('active', 'retired', 'revoked')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL
);

CREATE UNIQUE INDEX uq_signing_keys_kid
  ON signing_keys (kid)
  WHERE deleted_at IS NULL;

CREATE TABLE audit_events (
  id UUID PRIMARY KEY,
  event_type TEXT NOT NULL,
  actor_user_id UUID NULL,
  actor_client_id UUID NULL,
  target_type TEXT NOT NULL,
  target_id TEXT NOT NULL,
  ip_address INET NULL,
  user_agent TEXT NULL,
  metadata JSONB NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ NULL
);

CREATE INDEX idx_audit_events_actor_user_id ON audit_events (actor_user_id);
CREATE INDEX idx_audit_events_target_id ON audit_events (target_id);
CREATE INDEX idx_audit_events_created_at ON audit_events (created_at);
