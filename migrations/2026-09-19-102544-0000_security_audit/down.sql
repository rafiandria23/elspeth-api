-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS idx_audit_events_created_at;
DROP INDEX IF EXISTS idx_audit_events_target_id;
DROP INDEX IF EXISTS idx_audit_events_actor_user_id;
DROP TABLE IF EXISTS audit_events;

DROP INDEX IF EXISTS uq_signing_keys_kid;
DROP TABLE IF EXISTS signing_keys;
