-- mysql definitions
USE DATABASE indus_identity;

DROP TABLE IF EXISTS client_desc;
DROP TABLE IF EXISTS client_secret;
DROP TABLE IF EXISTS client_tokens;

CREATE TABLE IF NOT EXISTS client_secret (
    sln             BIGINT UNSIGNED UNIQUE,
        -- protocol
    client_id       BINARY(16) NOT NULL,
    client_secret   BINARY(32) UNIQUE,
        -- optional, 256-bit secret exposed as a hex string
        -- unique for multiple instances of a client using the same 'client_id'
    expires_at      TIMESTAMP NOT NULL,
        -- required, dynamic client registration. protocol.
    issued_at       TIMESTAMP NOT NULL,
        -- optional, dynamic client registration.
    secret_type     TINYINT NOT NULL DEFAULT 0,
        -- 1 = SYM, 2 = ASYM,
    algorithm       VARCHAR(24) NOT NULL,
    public_key      VARBINARY(512) DEFAULT NULL,
    material        VARBINARY(32) DEFAULT NULL
);
-- FOREIGN KEY (client_id) REFERNCES client_desc(client_id);

CREATE TABLE IF NOT EXISTS client_tokens (
    client_id      	      BINARY(12) NOT NULL UNIQUE,
    reg_access_token      TEXT,
        -- OAuth 2.0 bearer token.
        -- issued by AS thorugh the client registration endpoint.
        -- used to authenticate the caller while accessing client configuration endpoint.
    initial_access_token  TEXT
);

CREATE TABLE IF NOT EXISTS client_redirect_uri(
    client_desc_sln BIGINT UNSIGNED NOT NULL,
    uri             VARCHAR(3000) NOT NULL,
);

CREATE TABLE IF NOT EXISTS client_desc (
    sln BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    client_id BINARY(16) NOT NULL,
    client_name VARCHAR(64) NOT NULL, -- human readable
    client_type TINYINT NOT NULL DEFAULT 1,
        -- 'confidential' = 1, 'public' = 3,
    application_type TINYINT NOT NULL DEFAULT 1,
        -- 'web' = 1, 'native' = 3
    client_profile TINYINT NOT NULL DEFAULT 1,
        -- 'server-webapp' = 1, 'SPA' = 3, 'user-agent' = 5,
        -- 'mobile-native-app' = 7, 'machine-local-app' = 11
    health TINYINT NOT NULL DEFAULT 0,
        -- BAD=0, PENDING=1, APPROVED=3, ACTIVE=7,
        -- DEACTIVATED=11, LOCKED=13, BLOCKED=17, DELETED=41
    registration_type TINYINT NOT NULL DEFAULT 0,
        -- DYNAMIC = 1, FORM_POST = 5, ADMIN_CONSOLE = 11, CUSTOM = 23
        -- VALIDATED = 32,
        -- HAS_CLIENT_SECRET = 64
    software_id CHAR(36) CHARACTER SET ascii NOT NULL,
        -- UUID string
        -- need not be human readable; opaque to the client and authz server.
        -- unlike 'client_id' this value DOES NOT vary across instances.
    software_version TEXT NOT NULL,

    redirect_uris TEXT NOT NULL,
        -- required.
        -- table 'client_redirect_uri' stores the actual URL values.
    grant_types SET('authorization_code', 'implicit', 'password', 'client_credentials',
        'refresh_token',
        'urn:ietf:params:oauth:grant-type:jwt-bearer',
        'urn:ietf:params:oauth:grant-type:saml2-bearer')
        NOT NULL DEFAULT 'authorization_code',
        -- required. ref: RFC 7591 - OAuth 2.0 Dynamic Registration.
    response_types SET('code', 'token',
        'id_token', 'id_token token',
        'code id_token', 'code token',
        'code id_token token', 'code token id_token') NOT NULL DEFAULT 'code',
        -- required.
    logo_uri TEXT,
        -- optional.
        -- when present, MUST point to a valid image file.
        -- when present, server SHOULD display the image to the end-user during approval.
    client_uri TEXT,
        -- optional. home page of the client.
        -- when present, MUST point to a valid web page.
        -- server SHOULD present this to the user so the user can safely follow the link.
        -- RECOMMENDATION: use interstitial pages to reduce vulnerabilities.
    policy_uri TEXT,
        -- optional. informs the end user how the profile data will be used.
        -- when present, MUST point to a valid web page.
        -- server SHOULD present this to the user.
    tos_uri TEXT,
        -- optional. terms of service.
        -- when present, MUST point to a valid web page.
        -- server SHOULD present this to the user.
    contacts TEXT NOT NULL,
        -- optional. array of email addresses of people responsible for the client.
    scopes TEXT,
        -- space-separated list of scope values.
        -- if omitted, the authz server MAY register a client with a default set of scopes/

    needs_reg_config_access boolean DEFAULT FALSE,
    -- auth_method='none' when confidential=false
    -- 'client_secret_basic' is the default.
    -- 'none', 'client_secret_post',
    token_endpoint_auth_method VARCHAR(2048) NOT NULL DEFAULT 'client_secret_basic',
    -- UPDATE indus_client SET token_endpoint_auth_signing_alg = 'RS256' WHERE token_endpoint_auth_method = 'private_key_jwt';
    token_endpoint_auth_signing_alg VARCHAR(32) NOT NULL,
    client_secret_expires_at INTEGER NOT NULL DEFAULT 0,
    sector_identifier_uri  TEXT,
    jwks JSON,
        -- optional. client's JSON Web Key Set document.
        -- MUST be a valid JSON Web Key Set document value..
    jwks_uri VARCHAR(2048),
        -- optional. URL for the client's JSON Web Key Set. RFC7517.
        -- refers to a document containing client's public keys.
    request_uris TEXT,
    audience TEXT,
    subject_type VARCHAR(15),
    allowed_cors_origins TEXT,
    request_object_signing_alg VARCHAR(10) NOT NULL,
    userinfo_signed_response_alg VARCHAR(10) NOT NULL,
    frontchannel_logout_uri TEXT NULL,
    frontchannel_logout_session_required BOOL NOT NULL DEFAULT FALSE,
    post_logout_redirect_uris TEXT NULL,
    backchannel_logout_uri TEXT NULL,
    backchannel_logout_session_required BOOL NOT NULL DEFAULT FALSE,

    misc         TEXT,
    created_at   TIMESTAMP NOT NULL,
    updated_at   TIMESTAMP
);

CREATE UNIQUE INDEX index_client_desc_client_id ON client_desc(client_id);
