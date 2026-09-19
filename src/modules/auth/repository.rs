use diesel::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    core::{
        database::schema::{
            audit_events, identity_providers, oauth_client_redirect_uris, oauth_client_scopes,
            oauth_clients, oauth_grant_scopes, oauth_grants, oauth_refresh_tokens, oauth_scopes,
            signing_keys,
        },
        error::{ApiError, Result},
    },
    modules::auth::model::{
        AuditEvent, IdentityProvider, NewAuditEvent, NewIdentityProvider, NewOauthClient,
        NewOauthClientRedirectUri, NewOauthClientScope, NewOauthGrant, NewOauthGrantScope,
        NewOauthRefreshToken, NewOauthScope, NewSigningKey, OauthClient, OauthClientRedirectUri,
        OauthGrant, OauthRefreshToken, OauthScope, SigningKey,
    },
};

pub struct AuthRepository;

impl AuthRepository {
    // =========================================================================
    // Identity Providers
    // =========================================================================

    pub fn create_identity_provider(
        conn: &mut PgConnection,
        new_idp: &NewIdentityProvider,
    ) -> Result<IdentityProvider> {
        diesel::insert_into(identity_providers::table)
            .values(new_idp)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_identity_provider_by_key(
        conn: &mut PgConnection,
        key: &str,
    ) -> Result<Option<IdentityProvider>> {
        identity_providers::table
            .filter(identity_providers::key.eq(key))
            .filter(identity_providers::status.eq("active"))
            .filter(identity_providers::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn find_identity_provider_by_id(
        conn: &mut PgConnection,
        idp_id: Uuid,
    ) -> Result<Option<IdentityProvider>> {
        identity_providers::table
            .find(idp_id)
            .filter(identity_providers::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn list_active_identity_providers(
        conn: &mut PgConnection,
    ) -> Result<Vec<IdentityProvider>> {
        identity_providers::table
            .filter(identity_providers::status.eq("active"))
            .filter(identity_providers::deleted_at.is_null())
            .load(conn)
            .map_err(ApiError::Database)
    }

    // =========================================================================
    // OAuth Clients & Redirect URIs
    // =========================================================================

    pub fn create_client(
        conn: &mut PgConnection,
        new_client: &NewOauthClient,
    ) -> Result<OauthClient> {
        diesel::insert_into(oauth_clients::table)
            .values(new_client)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_client_by_client_id(
        conn: &mut PgConnection,
        client_id: &str,
    ) -> Result<Option<OauthClient>> {
        oauth_clients::table
            .filter(oauth_clients::client_id.eq(client_id))
            .filter(oauth_clients::status.eq("active"))
            .filter(oauth_clients::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn find_client_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Option<OauthClient>> {
        oauth_clients::table
            .find(id)
            .filter(oauth_clients::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn create_client_redirect_uri(
        conn: &mut PgConnection,
        new_uri: &NewOauthClientRedirectUri,
    ) -> Result<OauthClientRedirectUri> {
        diesel::insert_into(oauth_client_redirect_uris::table)
            .values(new_uri)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_redirect_uris_by_client_id(
        conn: &mut PgConnection,
        client_uuid: Uuid,
    ) -> Result<Vec<OauthClientRedirectUri>> {
        oauth_client_redirect_uris::table
            .filter(oauth_client_redirect_uris::client_id.eq(client_uuid))
            .filter(oauth_client_redirect_uris::deleted_at.is_null())
            .load(conn)
            .map_err(ApiError::Database)
    }

    pub fn verify_client_redirect_uri(
        conn: &mut PgConnection,
        client_uuid: Uuid,
        uri: &str,
    ) -> Result<bool> {
        let count: i64 = oauth_client_redirect_uris::table
            .filter(oauth_client_redirect_uris::client_id.eq(client_uuid))
            .filter(oauth_client_redirect_uris::redirect_uri.eq(uri))
            .filter(oauth_client_redirect_uris::deleted_at.is_null())
            .count()
            .get_result(conn)
            .map_err(ApiError::Database)?;

        Ok(count > 0)
    }

    // =========================================================================
    // OAuth Scopes & Scope Assignments
    // =========================================================================

    pub fn create_scope(conn: &mut PgConnection, new_scope: &NewOauthScope) -> Result<OauthScope> {
        diesel::insert_into(oauth_scopes::table)
            .values(new_scope)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_scope_by_name(
        conn: &mut PgConnection,
        scope_name: &str,
    ) -> Result<Option<OauthScope>> {
        oauth_scopes::table
            .filter(oauth_scopes::name.eq(scope_name))
            .filter(oauth_scopes::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn assign_scope_to_client(
        conn: &mut PgConnection,
        new_client_scope: &NewOauthClientScope,
    ) -> Result<()> {
        diesel::insert_into(oauth_client_scopes::table)
            .values(new_client_scope)
            .execute(conn)
            .map_err(ApiError::Database)?;

        Ok(())
    }

    pub fn find_scopes_for_client(
        conn: &mut PgConnection,
        client_uuid: Uuid,
    ) -> Result<Vec<OauthScope>> {
        oauth_client_scopes::table
            .inner_join(oauth_scopes::table)
            .filter(oauth_client_scopes::client_id.eq(client_uuid))
            .filter(oauth_client_scopes::deleted_at.is_null())
            .filter(oauth_scopes::deleted_at.is_null())
            .select(oauth_scopes::all_columns)
            .load(conn)
            .map_err(ApiError::Database)
    }

    // =========================================================================
    // OAuth Grants & Granted Scopes
    // =========================================================================

    pub fn create_grant(conn: &mut PgConnection, new_grant: &NewOauthGrant) -> Result<OauthGrant> {
        diesel::insert_into(oauth_grants::table)
            .values(new_grant)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_grant_by_user_and_client(
        conn: &mut PgConnection,
        user_uuid: Uuid,
        client_uuid: Uuid,
    ) -> Result<Option<OauthGrant>> {
        oauth_grants::table
            .filter(oauth_grants::user_id.eq(user_uuid))
            .filter(oauth_grants::client_id.eq(client_uuid))
            .filter(oauth_grants::status.eq("active"))
            .filter(oauth_grants::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn find_grant_by_id(conn: &mut PgConnection, grant_id: Uuid) -> Result<Option<OauthGrant>> {
        oauth_grants::table
            .find(grant_id)
            .filter(oauth_grants::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn revoke_grant(conn: &mut PgConnection, grant_id: Uuid) -> Result<()> {
        let now = OffsetDateTime::now_utc();

        diesel::update(oauth_grants::table.find(grant_id))
            .filter(oauth_grants::deleted_at.is_null())
            .set((
                oauth_grants::status.eq("revoked"),
                oauth_grants::revoked_at.eq(now),
                oauth_grants::updated_at.eq(now),
            ))
            .execute(conn)
            .map_err(ApiError::Database)?;

        Ok(())
    }

    pub fn create_grant_scope(
        conn: &mut PgConnection,
        new_grant_scope: &NewOauthGrantScope,
    ) -> Result<()> {
        diesel::insert_into(oauth_grant_scopes::table)
            .values(new_grant_scope)
            .execute(conn)
            .map_err(ApiError::Database)?;

        Ok(())
    }

    pub fn find_scopes_for_grant(
        conn: &mut PgConnection,
        grant_uuid: Uuid,
    ) -> Result<Vec<OauthScope>> {
        oauth_grant_scopes::table
            .inner_join(oauth_scopes::table)
            .filter(oauth_grant_scopes::grant_id.eq(grant_uuid))
            .filter(oauth_grant_scopes::deleted_at.is_null())
            .filter(oauth_scopes::deleted_at.is_null())
            .select(oauth_scopes::all_columns)
            .load(conn)
            .map_err(ApiError::Database)
    }

    // =========================================================================
    // Refresh Tokens & Rotation
    // =========================================================================

    pub fn create_refresh_token(
        conn: &mut PgConnection,
        new_token: &NewOauthRefreshToken,
    ) -> Result<OauthRefreshToken> {
        diesel::insert_into(oauth_refresh_tokens::table)
            .values(new_token)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_refresh_token_by_hash(
        conn: &mut PgConnection,
        hash: &[u8],
    ) -> Result<Option<OauthRefreshToken>> {
        oauth_refresh_tokens::table
            .filter(oauth_refresh_tokens::token_hash.eq(hash))
            .filter(oauth_refresh_tokens::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn consume_refresh_token(
        conn: &mut PgConnection,
        token_id: Uuid,
        replaced_by: Uuid,
    ) -> Result<()> {
        let now = OffsetDateTime::now_utc();

        diesel::update(oauth_refresh_tokens::table.find(token_id))
            .filter(oauth_refresh_tokens::deleted_at.is_null())
            .set((
                oauth_refresh_tokens::status.eq("consumed"),
                oauth_refresh_tokens::consumed_at.eq(now),
                oauth_refresh_tokens::replaced_by_id.eq(replaced_by),
                oauth_refresh_tokens::updated_at.eq(now),
            ))
            .execute(conn)
            .map_err(ApiError::Database)?;

        Ok(())
    }

    pub fn revoke_token_family(conn: &mut PgConnection, family_id: Uuid) -> Result<()> {
        let now = OffsetDateTime::now_utc();

        diesel::update(oauth_refresh_tokens::table)
            .filter(oauth_refresh_tokens::family_id.eq(family_id))
            .filter(oauth_refresh_tokens::deleted_at.is_null())
            .set((
                oauth_refresh_tokens::status.eq("revoked"),
                oauth_refresh_tokens::updated_at.eq(now),
            ))
            .execute(conn)
            .map_err(ApiError::Database)?;

        Ok(())
    }

    // =========================================================================
    // Signing Keys (JWT Management)
    // =========================================================================

    pub fn create_signing_key(
        conn: &mut PgConnection,
        new_key: &NewSigningKey,
    ) -> Result<SigningKey> {
        diesel::insert_into(signing_keys::table)
            .values(new_key)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_active_signing_key_by_use(
        conn: &mut PgConnection,
        use_purpose: &str,
    ) -> Result<Option<SigningKey>> {
        signing_keys::table
            .filter(signing_keys::use_purpose.eq(use_purpose))
            .filter(signing_keys::status.eq("active"))
            .filter(signing_keys::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn find_signing_key_by_kid(
        conn: &mut PgConnection,
        kid: &str,
    ) -> Result<Option<SigningKey>> {
        signing_keys::table
            .filter(signing_keys::kid.eq(kid))
            .filter(signing_keys::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn list_public_signing_keys(conn: &mut PgConnection) -> Result<Vec<SigningKey>> {
        signing_keys::table
            .filter(signing_keys::status.eq_any(["active", "retired"]))
            .filter(signing_keys::deleted_at.is_null())
            .load(conn)
            .map_err(ApiError::Database)
    }

    // =========================================================================
    // Audit Logging
    // =========================================================================

    pub fn create_audit_event(
        conn: &mut PgConnection,
        new_event: &NewAuditEvent,
    ) -> Result<AuditEvent> {
        diesel::insert_into(audit_events::table)
            .values(new_event)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn list_audit_events_for_actor(
        conn: &mut PgConnection,
        actor_user_id: Uuid,
        limit: i64,
    ) -> Result<Vec<AuditEvent>> {
        audit_events::table
            .filter(audit_events::actor_user_id.eq(actor_user_id))
            .filter(audit_events::deleted_at.is_null())
            .order(audit_events::created_at.desc())
            .limit(limit)
            .load(conn)
            .map_err(ApiError::Database)
    }
}
