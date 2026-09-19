use diesel::{pg::Pg, prelude::*};
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    core::database::schema::{
        audit_events, identity_providers, oauth_client_redirect_uris, oauth_client_scopes,
        oauth_clients, oauth_grant_scopes, oauth_grants, oauth_refresh_tokens, oauth_scopes,
        signing_keys,
    },
    modules::user::model::User,
};

// =============================================================================
// External Identity Providers (OIDC / OAuth2)
// =============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = identity_providers)]
#[diesel(check_for_backend(Pg))]
pub struct IdentityProvider {
    pub id: Uuid,
    pub key: String,
    pub display_name: String,
    pub protocol: String,
    pub issuer: Option<String>,
    pub client_id: String,
    #[serde(skip_serializing)]
    pub client_secret_ref: Option<String>,
    pub discovery_url: Option<String>,
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub userinfo_url: Option<String>,
    pub jwks_url: Option<String>,
    pub status: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = identity_providers)]
pub struct NewIdentityProvider<'a> {
    pub id: Uuid,
    pub key: &'a str,
    pub display_name: &'a str,
    pub protocol: &'a str,
    pub issuer: Option<&'a str>,
    pub client_id: &'a str,
    pub client_secret_ref: Option<&'a str>,
    pub discovery_url: Option<&'a str>,
    pub authorization_url: Option<&'a str>,
    pub token_url: Option<&'a str>,
    pub userinfo_url: Option<&'a str>,
    pub jwks_url: Option<&'a str>,
    pub status: &'a str,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

// =============================================================================
// OAuth Clients & Redirect URIs
// =============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = oauth_clients)]
#[diesel(check_for_backend(Pg))]
pub struct OauthClient {
    pub id: Uuid,
    pub client_id: String,
    #[serde(skip_serializing)]
    pub client_secret_hash: Option<String>,
    pub name: String,
    pub client_type: String,
    pub is_first_party: bool,
    pub status: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = oauth_clients)]
pub struct NewOauthClient<'a> {
    pub id: Uuid,
    pub client_id: &'a str,
    pub client_secret_hash: Option<&'a str>,
    pub name: &'a str,
    pub client_type: &'a str,
    pub is_first_party: bool,
    pub status: &'a str,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize,
)]
#[diesel(table_name = oauth_client_redirect_uris)]
#[diesel(belongs_to(OauthClient, foreign_key = client_id))]
#[diesel(check_for_backend(Pg))]
pub struct OauthClientRedirectUri {
    pub id: Uuid,
    pub client_id: Uuid,
    pub redirect_uri: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = oauth_client_redirect_uris)]
pub struct NewOauthClientRedirectUri<'a> {
    pub id: Uuid,
    pub client_id: Uuid,
    pub redirect_uri: &'a str,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

// =============================================================================
// OAuth Scopes & Client Scope Assignments
// =============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = oauth_scopes)]
#[diesel(check_for_backend(Pg))]
pub struct OauthScope {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub is_system: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = oauth_scopes)]
pub struct NewOauthScope<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub description: &'a str,
    pub is_system: bool,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize,
)]
#[diesel(table_name = oauth_client_scopes)]
#[diesel(primary_key(client_id, scope_id))]
#[diesel(belongs_to(OauthClient, foreign_key = client_id))]
#[diesel(belongs_to(OauthScope, foreign_key = scope_id))]
#[diesel(check_for_backend(Pg))]
pub struct OauthClientScope {
    pub client_id: Uuid,
    pub scope_id: Uuid,
    pub is_default: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = oauth_client_scopes)]
pub struct NewOauthClientScope {
    pub client_id: Uuid,
    pub scope_id: Uuid,
    pub is_default: bool,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

// =============================================================================
// OAuth User Grants & Granted Scopes
// =============================================================================

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize,
)]
#[diesel(table_name = oauth_grants)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(OauthClient, foreign_key = client_id))]
#[diesel(check_for_backend(Pg))]
pub struct OauthGrant {
    pub id: Uuid,
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub status: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub revoked_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = oauth_grants)]
pub struct NewOauthGrant<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub status: &'a str,
    pub revoked_at: Option<OffsetDateTime>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize,
)]
#[diesel(table_name = oauth_grant_scopes)]
#[diesel(primary_key(grant_id, scope_id))]
#[diesel(belongs_to(OauthGrant, foreign_key = grant_id))]
#[diesel(belongs_to(OauthScope, foreign_key = scope_id))]
#[diesel(check_for_backend(Pg))]
pub struct OauthGrantScope {
    pub grant_id: Uuid,
    pub scope_id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = oauth_grant_scopes)]
pub struct NewOauthGrantScope {
    pub grant_id: Uuid,
    pub scope_id: Uuid,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

// =============================================================================
// Refresh Tokens (Token Family Rotation Support)
// =============================================================================

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize,
)]
#[diesel(table_name = oauth_refresh_tokens)]
#[diesel(belongs_to(OauthGrant, foreign_key = grant_id))]
#[diesel(check_for_backend(Pg))]
pub struct OauthRefreshToken {
    pub id: Uuid,
    pub family_id: Uuid,
    #[serde(skip_serializing)]
    pub token_hash: Vec<u8>,
    pub grant_id: Uuid,
    pub status: String,
    pub granted_scopes: Vec<Option<String>>,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub consumed_at: Option<OffsetDateTime>,
    pub replaced_by_id: Option<Uuid>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = oauth_refresh_tokens)]
pub struct NewOauthRefreshToken<'a> {
    pub id: Uuid,
    pub family_id: Uuid,
    pub token_hash: &'a [u8],
    pub grant_id: Uuid,
    pub status: &'a str,
    pub granted_scopes: Vec<Option<String>>,
    pub expires_at: OffsetDateTime,
    pub consumed_at: Option<OffsetDateTime>,
    pub replaced_by_id: Option<Uuid>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

// =============================================================================
// Key Management (JWT Signing Keys)
// =============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = signing_keys)]
#[diesel(check_for_backend(Pg))]
pub struct SigningKey {
    pub id: Uuid,
    pub kid: String,
    pub use_purpose: String,
    pub algorithm: String,
    pub public_jwk: JsonValue,
    #[serde(skip_serializing)]
    pub private_key_ref: String,
    pub status: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = signing_keys)]
pub struct NewSigningKey<'a> {
    pub id: Uuid,
    pub kid: &'a str,
    pub use_purpose: &'a str,
    pub algorithm: &'a str,
    pub public_jwk: JsonValue,
    pub private_key_ref: &'a str,
    pub status: &'a str,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

// =============================================================================
// Immutable Audit Logging
// =============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = audit_events)]
#[diesel(check_for_backend(Pg))]
pub struct AuditEvent {
    pub id: Uuid,
    pub event_type: String,
    pub actor_user_id: Option<Uuid>,
    pub actor_client_id: Option<Uuid>,
    pub target_type: String,
    pub target_id: String,
    pub ip_address: Option<IpNetwork>,
    pub user_agent: Option<String>,
    pub metadata: Option<JsonValue>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = audit_events)]
pub struct NewAuditEvent<'a> {
    pub id: Uuid,
    pub event_type: &'a str,
    pub actor_user_id: Option<Uuid>,
    pub actor_client_id: Option<Uuid>,
    pub target_type: &'a str,
    pub target_id: &'a str,
    pub ip_address: Option<IpNetwork>,
    pub user_agent: Option<&'a str>,
    pub metadata: Option<JsonValue>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}
