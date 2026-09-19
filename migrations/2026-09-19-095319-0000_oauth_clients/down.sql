-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS idx_oauth_client_scopes_scope_id;
DROP TABLE IF EXISTS oauth_client_scopes;

DROP INDEX IF EXISTS uq_oauth_scope_name;
DROP TABLE IF EXISTS oauth_scopes;

DROP INDEX IF EXISTS idx_oauth_client_redirect_uris_client_id;
DROP INDEX IF EXISTS uq_oauth_client_redirect_uris_client_redirect;
DROP TABLE IF EXISTS oauth_client_redirect_uris;

DROP INDEX IF EXISTS uq_oauth_clients_client_id;
DROP TABLE IF EXISTS oauth_clients;
