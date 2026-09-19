-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS idx_federated_identities_user_id;
DROP INDEX IF EXISTS uq_federated_identities_provider_subject;
DROP TABLE IF EXISTS federated_identities;

DROP INDEX IF EXISTS uq_identity_providers_key;
DROP TABLE IF EXISTS identity_providers;
