use diesel::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    core::{
        database::schema::{
            authenticators, credentials, federated_identities, sessions, sign_in_identifiers, users,
        },
        error::{ApiError, Result},
    },
    modules::user::model::{
        Authenticator, Credential, FederatedIdentity, NewAuthenticator, NewCredential,
        NewFederatedIdentity, NewSession, NewSignInIdentifier, NewUser, Session, SignInIdentifier,
        User,
    },
};

pub struct UserRepository;

impl UserRepository {
    // =========================================================================
    // Core User Operations
    // =========================================================================

    pub fn create_user(conn: &mut PgConnection, new_user: &NewUser) -> Result<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> Result<Option<User>> {
        users::table
            .find(user_id)
            .filter(users::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn update_user_status(
        conn: &mut PgConnection,
        user_id: Uuid,
        new_status: &str,
    ) -> Result<User> {
        let now = OffsetDateTime::now_utc();

        diesel::update(users::table.find(user_id))
            .filter(users::deleted_at.is_null())
            .set((users::status.eq(new_status), users::updated_at.eq(now)))
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn soft_delete_user(conn: &mut PgConnection, user_id: Uuid) -> Result<usize> {
        let now = OffsetDateTime::now_utc();

        diesel::update(users::table.find(user_id))
            .filter(users::deleted_at.is_null())
            .set((users::deleted_at.eq(now), users::updated_at.eq(now)))
            .execute(conn)
            .map_err(ApiError::Database)
    }

    // =========================================================================
    // Sign-In Identifiers
    // =========================================================================

    pub fn create_sign_in_identifier(
        conn: &mut PgConnection,
        new_identifier: &NewSignInIdentifier,
    ) -> Result<SignInIdentifier> {
        diesel::insert_into(sign_in_identifiers::table)
            .values(new_identifier)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_identifier_by_normalized(
        conn: &mut PgConnection,
        identifier_type: &str,
        normalized: &str,
    ) -> Result<Option<SignInIdentifier>> {
        sign_in_identifiers::table
            .filter(sign_in_identifiers::type_.eq(identifier_type))
            .filter(sign_in_identifiers::normalized_identifier.eq(normalized))
            .filter(sign_in_identifiers::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn find_identifiers_by_user_id(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<Vec<SignInIdentifier>> {
        sign_in_identifiers::table
            .filter(sign_in_identifiers::user_id.eq(user_id))
            .filter(sign_in_identifiers::deleted_at.is_null())
            .load(conn)
            .map_err(ApiError::Database)
    }

    pub fn mark_identifier_verified(
        conn: &mut PgConnection,
        identifier_id: Uuid,
    ) -> Result<SignInIdentifier> {
        let now = OffsetDateTime::now_utc();

        diesel::update(sign_in_identifiers::table.find(identifier_id))
            .filter(sign_in_identifiers::deleted_at.is_null())
            .set((
                sign_in_identifiers::verified_at.eq(now),
                sign_in_identifiers::updated_at.eq(now),
            ))
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    // =========================================================================
    // Credentials (Passwords)
    // =========================================================================

    pub fn create_credential(
        conn: &mut PgConnection,
        new_cred: &NewCredential,
    ) -> Result<Credential> {
        diesel::insert_into(credentials::table)
            .values(new_cred)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_active_credential_by_user_id(
        conn: &mut PgConnection,
        user_id: Uuid,
        cred_type: &str,
    ) -> Result<Option<Credential>> {
        credentials::table
            .filter(credentials::user_id.eq(user_id))
            .filter(credentials::type_.eq(cred_type))
            .filter(credentials::status.eq("active"))
            .filter(credentials::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn touch_credential_last_used(conn: &mut PgConnection, credential_id: Uuid) -> Result<()> {
        let now = OffsetDateTime::now_utc();

        diesel::update(credentials::table.find(credential_id))
            .filter(credentials::deleted_at.is_null())
            .set((
                credentials::last_used_at.eq(now),
                credentials::updated_at.eq(now),
            ))
            .execute(conn)
            .map_err(ApiError::Database)?;

        Ok(())
    }

    // =========================================================================
    // Authenticators (WebAuthn / Passkeys)
    // =========================================================================

    pub fn create_authenticator(
        conn: &mut PgConnection,
        new_auth: &NewAuthenticator,
    ) -> Result<Authenticator> {
        diesel::insert_into(authenticators::table)
            .values(new_auth)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_authenticator_by_credential_id(
        conn: &mut PgConnection,
        cred_id: &str,
    ) -> Result<Option<Authenticator>> {
        authenticators::table
            .filter(authenticators::credential_id.eq(cred_id))
            .filter(authenticators::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn update_authenticator_sign_count(
        conn: &mut PgConnection,
        auth_id: Uuid,
        new_count: i64,
    ) -> Result<()> {
        let now = OffsetDateTime::now_utc();

        diesel::update(authenticators::table.find(auth_id))
            .filter(authenticators::deleted_at.is_null())
            .set((
                authenticators::sign_count.eq(new_count),
                authenticators::last_used_at.eq(now),
                authenticators::updated_at.eq(now),
            ))
            .execute(conn)
            .map_err(ApiError::Database)?;

        Ok(())
    }

    // =========================================================================
    // Sessions
    // =========================================================================

    pub fn create_session(conn: &mut PgConnection, new_sess: &NewSession) -> Result<Session> {
        diesel::insert_into(sessions::table)
            .values(new_sess)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_session_by_hash(conn: &mut PgConnection, hash: &[u8]) -> Result<Option<Session>> {
        sessions::table
            .filter(sessions::session_hash.eq(hash))
            .filter(sessions::status.eq("active"))
            .filter(sessions::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }

    pub fn touch_session(conn: &mut PgConnection, session_id: Uuid) -> Result<()> {
        let now = OffsetDateTime::now_utc();

        diesel::update(sessions::table.find(session_id))
            .filter(sessions::deleted_at.is_null())
            .set((sessions::last_seen_at.eq(now), sessions::updated_at.eq(now)))
            .execute(conn)
            .map_err(ApiError::Database)?;

        Ok(())
    }

    pub fn revoke_session(conn: &mut PgConnection, session_id: Uuid) -> Result<()> {
        let now = OffsetDateTime::now_utc();

        diesel::update(sessions::table.find(session_id))
            .filter(sessions::deleted_at.is_null())
            .set((sessions::status.eq("revoked"), sessions::updated_at.eq(now)))
            .execute(conn)
            .map_err(ApiError::Database)?;

        Ok(())
    }

    // =========================================================================
    // Federated Identities
    // =========================================================================

    pub fn create_federated_identity(
        conn: &mut PgConnection,
        new_fed: &NewFederatedIdentity,
    ) -> Result<FederatedIdentity> {
        diesel::insert_into(federated_identities::table)
            .values(new_fed)
            .get_result(conn)
            .map_err(ApiError::Database)
    }

    pub fn find_federated_identity(
        conn: &mut PgConnection,
        provider_id: Uuid,
        provider_subject: &str,
    ) -> Result<Option<FederatedIdentity>> {
        federated_identities::table
            .filter(federated_identities::provider_id.eq(provider_id))
            .filter(federated_identities::provider_subject.eq(provider_subject))
            .filter(federated_identities::deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(ApiError::Database)
    }
}
