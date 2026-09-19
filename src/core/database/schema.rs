// @generated automatically by Diesel CLI.

diesel::table! {
    audit_events (id) {
        id -> Uuid,
        event_type -> Text,
        actor_user_id -> Nullable<Uuid>,
        actor_client_id -> Nullable<Uuid>,
        target_type -> Text,
        target_id -> Text,
        ip_address -> Nullable<Inet>,
        user_agent -> Nullable<Text>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    authenticators (id) {
        id -> Uuid,
        user_id -> Uuid,
        credential_id -> Text,
        public_key -> Bytea,
        sign_count -> Int8,
        transports -> Nullable<Jsonb>,
        name -> Nullable<Text>,
        last_used_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    credentials (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[sql_name = "type"]
        type_ -> Text,
        secret_hash -> Text,
        status -> Text,
        last_used_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    federated_identities (id) {
        id -> Uuid,
        user_id -> Uuid,
        provider_id -> Uuid,
        provider_subject -> Text,
        last_used_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    identity_providers (id) {
        id -> Uuid,
        key -> Text,
        display_name -> Text,
        protocol -> Text,
        issuer -> Nullable<Text>,
        client_id -> Text,
        client_secret_ref -> Nullable<Text>,
        discovery_url -> Nullable<Text>,
        authorization_url -> Nullable<Text>,
        token_url -> Nullable<Text>,
        userinfo_url -> Nullable<Text>,
        jwks_url -> Nullable<Text>,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    oauth_client_redirect_uris (id) {
        id -> Uuid,
        client_id -> Uuid,
        redirect_uri -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    oauth_client_scopes (client_id, scope_id) {
        client_id -> Uuid,
        scope_id -> Uuid,
        is_default -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    oauth_clients (id) {
        id -> Uuid,
        client_id -> Text,
        client_secret_hash -> Nullable<Text>,
        name -> Text,
        client_type -> Text,
        is_first_party -> Bool,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    oauth_grant_scopes (grant_id, scope_id) {
        grant_id -> Uuid,
        scope_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    oauth_grants (id) {
        id -> Uuid,
        user_id -> Uuid,
        client_id -> Uuid,
        status -> Text,
        revoked_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    oauth_refresh_tokens (id) {
        id -> Uuid,
        family_id -> Uuid,
        token_hash -> Bytea,
        grant_id -> Uuid,
        status -> Text,
        granted_scopes -> Array<Nullable<Text>>,
        expires_at -> Timestamptz,
        consumed_at -> Nullable<Timestamptz>,
        replaced_by_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    oauth_scopes (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        is_system -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        session_hash -> Bytea,
        status -> Text,
        ip_address -> Nullable<Inet>,
        user_agent -> Nullable<Text>,
        expires_at -> Timestamptz,
        last_seen_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    sign_in_identifiers (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[sql_name = "type"]
        type_ -> Text,
        identifier -> Text,
        normalized_identifier -> Text,
        is_primary -> Bool,
        verified_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    signing_keys (id) {
        id -> Uuid,
        kid -> Text,
        use_purpose -> Text,
        algorithm -> Text,
        public_jwk -> Jsonb,
        private_key_ref -> Text,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(authenticators -> users (user_id));
diesel::joinable!(credentials -> users (user_id));
diesel::joinable!(federated_identities -> identity_providers (provider_id));
diesel::joinable!(federated_identities -> users (user_id));
diesel::joinable!(oauth_client_redirect_uris -> oauth_clients (client_id));
diesel::joinable!(oauth_client_scopes -> oauth_clients (client_id));
diesel::joinable!(oauth_client_scopes -> oauth_scopes (scope_id));
diesel::joinable!(oauth_grant_scopes -> oauth_grants (grant_id));
diesel::joinable!(oauth_grant_scopes -> oauth_scopes (scope_id));
diesel::joinable!(oauth_grants -> oauth_clients (client_id));
diesel::joinable!(oauth_grants -> users (user_id));
diesel::joinable!(oauth_refresh_tokens -> oauth_grants (grant_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(sign_in_identifiers -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    audit_events,
    authenticators,
    credentials,
    federated_identities,
    identity_providers,
    oauth_client_redirect_uris,
    oauth_client_scopes,
    oauth_clients,
    oauth_grant_scopes,
    oauth_grants,
    oauth_refresh_tokens,
    oauth_scopes,
    sessions,
    sign_in_identifiers,
    signing_keys,
    users,
);
