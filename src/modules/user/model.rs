use diesel::{pg::Pg, prelude::*};
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::core::database::schema::{
    authenticators, credentials, federated_identities, sessions, sign_in_identifiers, users,
};

// =============================================================================
// Core Identity: User
// =============================================================================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: Uuid,
    pub status: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub status: &'a str,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

// =============================================================================
// Sign-In Identifiers (Email, Username, etc.)
// =============================================================================

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize,
)]
#[diesel(table_name = sign_in_identifiers)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
pub struct SignInIdentifier {
    pub id: Uuid,
    pub user_id: Uuid,
    pub type_: String,
    pub identifier: String,
    pub normalized_identifier: String,
    pub is_primary: bool,
    #[serde(with = "time::serde::rfc3339::option")]
    pub verified_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sign_in_identifiers)]
pub struct NewSignInIdentifier<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub type_: &'a str,
    pub identifier: &'a str,
    pub normalized_identifier: &'a str,
    pub is_primary: bool,
    pub verified_at: Option<OffsetDateTime>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

// =============================================================================
// Primary Credentials (Password Hashes)
// =============================================================================

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize,
)]
#[diesel(table_name = credentials)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
pub struct Credential {
    pub id: Uuid,
    pub user_id: Uuid,
    pub type_: String,
    #[serde(skip_serializing)]
    pub secret_hash: String,
    pub status: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_used_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = credentials)]
pub struct NewCredential<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub type_: &'a str,
    pub secret_hash: &'a str,
    pub status: &'a str,
    pub last_used_at: Option<OffsetDateTime>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

// =============================================================================
// WebAuthn / FIDO2 Authenticators
// =============================================================================

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize,
)]
#[diesel(table_name = authenticators)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
pub struct Authenticator {
    pub id: Uuid,
    pub user_id: Uuid,
    pub credential_id: String,
    pub public_key: Vec<u8>,
    pub sign_count: i64,
    pub transports: Option<JsonValue>,
    pub name: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_used_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = authenticators)]
pub struct NewAuthenticator<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub credential_id: &'a str,
    pub public_key: &'a [u8],
    pub sign_count: i64,
    pub transports: Option<JsonValue>,
    pub name: Option<&'a str>,
    pub last_used_at: Option<OffsetDateTime>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

// =============================================================================
// User Sessions
// =============================================================================

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize,
)]
#[diesel(table_name = sessions)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    #[serde(skip_serializing)]
    pub session_hash: Vec<u8>,
    pub status: String,
    pub ip_address: Option<IpNetwork>,
    pub user_agent: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub last_seen_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sessions)]
pub struct NewSession<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_hash: &'a [u8],
    pub status: &'a str,
    pub ip_address: Option<IpNetwork>,
    pub user_agent: Option<&'a str>,
    pub expires_at: OffsetDateTime,
    pub last_seen_at: OffsetDateTime,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}

// =============================================================================
// Federated Identities (SSO Links)
// =============================================================================

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize,
)]
#[diesel(table_name = federated_identities)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
pub struct FederatedIdentity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider_id: Uuid,
    pub provider_subject: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_used_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = federated_identities)]
pub struct NewFederatedIdentity<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider_id: Uuid,
    pub provider_subject: &'a str,
    pub last_used_at: Option<OffsetDateTime>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
}
