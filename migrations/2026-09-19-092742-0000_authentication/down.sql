-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS idx_sessions_user_id;
DROP INDEX IF EXISTS uq_sessions_session_hash;
DROP TABLE IF EXISTS sessions;

DROP INDEX IF EXISTS idx_authenticators_user_id;
DROP INDEX IF EXISTS uq_authenticators_credential_id;
DROP TABLE IF EXISTS authenticators;

DROP INDEX IF EXISTS dx_credentials_user_id;
DROP TABLE IF EXISTS credentials;
