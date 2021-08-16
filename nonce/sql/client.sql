-- mysql definitions
USE DATABASE indus_oauth;

DROP TABLE IF EXISTS client_desc;
DROP TABLE IF EXISTS client_secret;
DROP TABLE IF EXISTS client_tokens;

CREATE TABLE IF NOT EXISTS client_secret (
    sln             BIGINT UNSIGNED UNIQUE,
        -- protocol
    client_id       BINARY(12) NOT NULL,
    client_secret   BINARY(32) UNIQUE,
        -- optional, 256-bit secrete exposed as a hex string
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
FOREIGN KEY (client_id) REFERNCES client_desc(client_id);

CREATE TABLE IF NOT EXISTS client_tokens (
    client_id      	      BINARY(12) NOT NULL UNIQUE,
    reg_access_token      TEXT,
        -- OAuth 2.0 bearer token.
        -- issued by AS thorugh the client registration endpoint.
        -- used to authenticate the caller while accessing client configuration endpoint.
    initial_access_token  TEXT
);

CREATE TABLE IF NOT EXISTS client_desc (
    sln               BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    client_id      	  BINARY(12) NOT NULL,
    client_name       TEXT(224) COLLATE utf8mb4_0900_bin,
    client_type       TINYINT NOT NULL DEFAULT 1,
        -- 'confidential' = 1, 'public' = 2,
    application_type  TINYINT NOT NULL DEFAULT 1,
        -- 'web' = 1, 'native' = 2
    client_profile    TINYINT NOT NULL DEFAULT 1,
        -- 'server-webapp' = 1, 'SPA' = 2, 'user-agent' = 6,
        -- 'mobile-native-app' = 12, 'machine-local-app' = 18
        -- 'pkce-aware' = 24
    status            TINYINT NOT NULL DEFAULT 0,
        -- BAD=0, PENDING=1, APPROVED=2, ACTIVE=4,
        -- DEACTIVATED=8, LOCKED=16, BLOCKED=32, DELETED=64
    registration_type  SMALLINT NOT NULL DEFAULT 0,
        -- DYNAMIC = 1, FORM_POST = 2, ADMIN_CONSOLE = 4, CUSTOM = 8
        -- VALIDATED = 1 << 8,
        -- HAS_CLIENT_SECRET = 1 << 13
    redirect_uris  	  TEXT NOT NULL,
        -- required.
    grant_types       SMALLINT NOT NULL DEFAULT 1,
        -- required. ref: RFC 7591 - OAuth 2.0 Dynamic Registration.
        -- 'authorization_code' = 1, 'password' = 2, 'client_credentials' = 4,
        -- 'pkce' = 8, 'refresh_token' = 16, 'implicit' = 32
        -- 'jwt-bearer' = 64, 'saml2-bearer' = 128
    response_types    SMALLINT NOT NULL DEFAULT 1,
        -- required.
        -- 'code' = 1, 'token' = 2, 'id_token '= 4
    logo_uri  		  TEXT,
        -- optional.
        -- when present, MUST point to a valid image file.
        -- when present, server SHOULD display the image to the end-user during approval.
    client_uri  	  TEXT,
        -- optional. home page of the client.
        -- when present, MUST point to a valid web page.
        -- server SHOULD present this to the user so the user can safely follow the link.
        -- RECOMMENDATION: use interstitial pages to reduce vulnerabilities.
    policy_uri        TEXT,
        -- optional. informs the end user how the profile data will be used.
        -- when present, MUST point to a valid web page.
        -- server SHOULD present this to the user.
    tos_uri           TEXT,
        -- optional. terms of service.
        -- when present, MUST point to a valid web page.
        -- server SHOULD present this to the user.
    contacts          TEXT NOT NULL,
        -- optional. array of email addresses of people responsible for the client.
    scopes            TEXT,

    needs_reg_config_access boolean DEFAULT FALSE,
    -- auth_method='none' when confidential=false
    token_endpoint_auth_method   VARCHAR(32) NOT NULL,
    -- UPDATE indus_client SET token_endpoint_auth_signing_alg = 'RS256' WHERE token_endpoint_auth_method = 'private_key_jwt';
    token_endpoint_auth_signing_alg VARCHAR(32) NOT NULL,
    client_secret_expires_at     INTEGER NOT NULL DEFAULT 0,
    sector_identifier_uri  TEXT,
    jwks                 VARCHAR(2048),
        -- optional. client's JSON Web Key Set document.
    jwks_uri             VARCHAR(2048),
        -- optional. URL for the client's JSON Web Key Set.
    request_uris         TEXT,
    audience             TEXT,
    subject_type         VARCHAR(15),
    allowed_cors_origins TEXT,
    request_object_signing_alg    VARCHAR(10) NOT NULL,
    userinfo_signed_response_alg  VARCHAR(10) NOT NULL,
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
