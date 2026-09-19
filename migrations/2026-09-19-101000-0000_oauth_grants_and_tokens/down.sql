-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS idx_oauth_refresh_tokens_grant_id;
DROP INDEX IF EXISTS idx_oauth_refresh_tokens_family_id;
DROP INDEX IF EXISTS uq_oauth_refresh_tokens_token_hash;
DROP TABLE IF EXISTS oauth_refresh_tokens;

DROP INDEX IF EXISTS idx_oauth_grant_scopes_scope_id;
DROP TABLE IF EXISTS oauth_grant_scopes;

DROP INDEX IF EXISTS idx_oauth_grants_client_id;
DROP INDEX IF EXISTS idx_oauth_grants_user_id;
DROP INDEX IF EXISTS uq_oauth_grants_user_client;
DROP TABLE IF EXISTS oauth_grants;
