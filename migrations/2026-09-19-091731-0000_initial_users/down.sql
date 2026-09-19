-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS idx_sign_in_identifiers_user_id;
DROP INDEX IF EXISTS uq_sign_in_identifiers_user_primary_type;
DROP INDEX IF EXISTS uq_sign_in_identifiers_type_normalized;
DROP TABLE IF EXISTS sign_in_identifiers;

DROP TABLE IF EXISTS users;
