-- mysql definitions
USE DATABASE indus_identity;

DROP TABLE IF EXISTS client_desc;
DROP TABLE IF EXISTS client_tokens;

DROP TABLE IF EXISTS client_secret;
CREATE TABLE IF NOT EXISTS client_secret (
    clid_rep BINARY(32) NOT NULL,
    client_id VARCHAR(256) NOT NULL,
        -- unique for each instance of a client with the same 'clid_rep'
    UNIQUE (clid_rep, client_id),
    client_secret BINARY(32) UNIQUE,
    UNIQUE (client_secret),
    client_passwd BINARY(32) UNIQUE,
    UNIQUE (client_passwd),
    client_secret_iat INTEGER NOT NULL DEFAULT 0,
        -- RFC7591 - after a successful dynamic registration request.
        -- Time at which the client identifier was issued.  The time is represented
        --     as the number of seconds from 1970-01-01T00:00:00Z as measured in UTC
        --     until the date/time of issuance.
    client_secret_xat INTEGER NOT NULL DEFAULT 0,
        -- RFC7591 - after a successful dynamic registration request.
        -- REQUIRED if "client_secret" is issued.
        -- Time at which the client secret will expire or 0 if it will not expire.
        -- The time is represented as the number of seconds from 1970-01-01T00:00:00Z as
        --     measured in UTC until the date/time of expiration.
    client_passwd_iat INTEGER NOT NULL DEFAULT 0,
    client_passwd_xat INTEGER NOT NULL DEFAULT 0
);
-- FOREIGN KEY (client_id) REFERNCES client_desc(client_id);

CREATE TABLE IF NOT EXISTS client_reg_token (
    clid_rep BINARY(32) NOT NULL,
    client_id VARCHAR(256) NOT NULL,
    UNIQUE (clid_rep, client_id),
    initial_access_token TEXT,
    init_access_tok_iat TIMESTAMP NOT NULL,
    init_access_tok_xat TIMESTAMP NOT NULL,
    reg_access_token TEXT,
    reg_access_tok_iat TIMESTAMP NOT NULL,
    reg_access_tok_xat TIMESTAMP NOT NULL
        -- OAuth 2.0 bearer token.
        -- issued by AS thorugh the client registration endpoint.
        -- used to authenticate the caller while accessing client configuration endpoint.
);

CREATE TABLE IF NOT EXISTS client_redirect_uri (
    client_id VARCHAR(256) NOT NULL,
    uri VARCHAR(2048) CHARACTER SET ascii NOT NULL,
    UNIQUE (client_id, uri)
);

CREATE TABLE IF NOT EXISTS client_event (
    client_id VARCHAR(256) NOT NULL,
    msg TEXT NOT NULL,
    evt TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS client_desc (
    sln SERIAL PRIMARY KEY,
    clid_rep BINARY(32) NOT NULL,
        -- unique id for cleints within this realm.
        -- identify the non-varying component for dynamically registered client instances.
        -- this is used in additional to 'software_id' when available.
    client_id VARCHAR(256) NOT NULL,
    UNIQUE (clid_rep, client_id),
    health TINYINT NOT NULL DEFAULT 0,
        -- PENDING = 0, REGISTERED = 2, ACTIVE = 4,
        -- COMPROMISED = 8, DEACTIVATED = 16, LOCKED = 32,
        -- ZOMBIE = 128
    client_profile TINYINT NOT NULL DEFAULT 1,
        -- 'web-confidential' = 1,
        -- 'web-public' = 5,
        -- 'user-agent' = 11, 'SPA' = 19,
        -- 'device-native' = 29,
        -- 'localhost-confidential' = 37,
        -- 'localhost-public' = 39,
    registration_type TINYINT NOT NULL DEFAULT 0,
        -- DYNAMIC = 1, FORM_POST = 5, ADMIN_CONSOLE = 11, CUSTOM = 23
        -- VALIDATED = 32,
        -- HAS_CLIENT_SECRET = 64

    client_type TINYINT NOT NULL DEFAULT 1,
        -- 'confidential' = 1, 'public' = 3,
    client_name VARCHAR(64) NOT NULL,
         -- RECOMMENDED. internationized. human readable - presented to the End-User.
    application_type TINYINT NOT NULL DEFAULT 1,
        -- 'web' = 1, 'native' = 3
        -- oidc. optional, Indus:MANDATORY.
    software_id CHAR(36) CHARACTER SET ascii NOT NULL,
        -- oauth 2.0. UUID string
        -- need not be human readable; opaque to the client and authz server.
        -- unlike 'client_id' this value DOES NOT vary across instances.
    software_version VARCHAR(32) CHARACTER SET ascii NOT NULL,
        -- oauth 2.0. string equality matching - no other comparison semantics.
    redirect_uris TEXT CHARACTER SET ascii NOT NULL,
        -- required.
        -- table 'client_redirect_uri' stores the actual URL values.
    grant_types VARHAR(64) NOT NULL DEFAULT 'authorization_code',
        -- 'authorization_code', 'implicit', 'refresh_token', -- OAuth 2.0 and OIDC
        -- 'password', 'client_credentials',
        -- 'urn:ietf:params:oauth:grant-type:jwt-bearer',
        -- 'urn:ietf:params:oauth:grant-type:saml2-bearer',
        -- 'urn:openid:params:grant-type:ciba')
        -- NOT NULL DEFAULT 'authorization_code',
        -- required. ref: RFC 7591 - OAuth 2.0 Dynamic Registration.
        -- https://openid.net/specs/openid-client-initiated-backchannel-authentication-core-1_0.html
    response_types VARHAR(32) NOT NULL DEFAULT 'code',
        -- ('code', 'token',
        -- 'id_token', 'id_token token',
        -- 'code id_token', 'code token',
        -- 'code id_token token', 'code token id_token') NOT NULL DEFAULT 'code',
        -- required.
    logo_uri TEXT CHARACTER SET ascii,
        -- optional.
        -- when present, MUST point to a valid image file.
        -- when present, server SHOULD display the image to the end-user during approval.
    client_uri TEXT CHARACTER SET ascii,
        -- optional. home page of the client.
        -- when present, MUST point to a valid web page.
        -- server SHOULD present this to the user so the user can safely follow the link.
        -- RECOMMENDATION: use interstitial pages to reduce vulnerabilities.
    policy_uri TEXT CHARACTER SET ascii,
        -- optional. informs the end user how the profile data will be used.
        -- when present, MUST point to a valid web page.
        -- server SHOULD present this to the user.
    tos_uri TEXT CHARACTER SET ascii,
        -- optional. terms of service.
        -- when present, MUST point to a valid web page.
        -- server SHOULD present this to the user.
    contacts TEXT CHARACTER SET ascii NOT NULL,
        -- optional. array of email addresses of people responsible for the client.
        -- oidc states these are email addresses.
        -- oauth 2.0 dynamic registration (RFC7591) is not specific about the values.
    scope TEXT NOT NULL,
        -- space-separated list of scope values.
        -- if omitted, the authz server MAY register a client with a default set of scopes.
        -- Indus has defaults: 'aadhaar', 'voterid', 'pan', 'indus_id', 'indus_key'
    jwks_uri VARCHAR(2048),
        -- optional. URL for the client's JSON Web Key Set. RFC7517.
        -- refers to a document containing client's public keys.
        -- must follow requirements in RFC7591 and OIDC Dynamic Client Registration.
    jwks JSON,
        -- optional. client's JSON Web Key Set document.
        -- MUST be a valid JSON Web Key Set document value.
        -- must follow requirements in RFC7591 and OIDC Dynamic Client Registration.
    sector_identifier_uri  TEXT,
        -- OIDC Dynamic Client Registration.
        -- The URL references a file with a single JSON array of redirect_uri values.
        -- used in calculating pairwise identifiers.
    subject_type CHAR(8) CHARACTER SET ascii NOT NULL DEFAULT 'public',
        -- 'pairwise' OR 'public',
    id_token_signed_response_alg VARCHAR(22) CHARACTER SET ascii DEFAULT 'RS256',
        -- JWS alg algorithm [JWA] REQUIRED for signing the ID Token issued to this Client.
        -- The public key for validating the signature is provided by retrieving the
        --     JWK Set referenced by the jwks_uri element from OpenID Connect Discovery 1.0.
        -- 'none' JWS alg algorithm value.
    id_token_encrypted_response_alg VARCHAR(22) CHARACTER SET ascii DEFAULT '',
    id_token_encrypted_response_enc VARCHAR(22) CHARACTER SET ascii DEFAULT '',
        -- When id_token_encrypted_response_enc is included,
        --    id_token_encrypted_response_alg MUST also be provided.
        -- If id_token_encrypted_response_alg is specified, the default for this
        --    value is 'A128CBC-HS256'.
    userinfo_signed_response_alg VARCHAR(22) CHARACTER SET ascii DEFAULT '',
        -- The default, if omitted, is for the UserInfo Response to return the
        --    Claims as a UTF-8 encoded JSON object using the application/json content-type.
    userinfo_encrypted_response_alg VARCHAR(22) CHARACTER SET ascii DEFAULT '',
    userinfo_encrypted_response_enc VARCHAR(22) CHARACTER SET ascii DEFAULT '',
        -- If userinfo_encrypted_response_alg is specified, the default for this
        --    value is 'A128CBC-HS256'.
        -- When userinfo_encrypted_response_enc is included,
        --    userinfo_encrypted_response_alg MUST also be provided.
    request_object_signing_alg VARCHAR(22) CHARACTER SET ascii DEFAULT 'none',
        -- JWS alg algorithm [JWA] that MUST be used for signing Request Objects
        --    sent to the OP. All Request Objects from this Client MUST be rejected,
        --    if not signed with this algorithm.
        -- Servers SHOULD support RS256. The value 'none' MAY be used. The default,
        --    if omitted, is that any algorithm supported by the OP and the RP MAY be used.
    request_object_encryption_alg VARCHAR(22) CHARACTER SET ascii DEFAULT '',
    request_object_encryption_enc VARCHAR(22) CHARACTER SET ascii DEFAULT '',
        -- If request_object_encryption_alg is specified, the default for this
        --     value is 'A128CBC-HS256'.
    needs_reg_config_access boolean DEFAULT FALSE,
        -- auth_method='none' when confidential=false
        -- 'client_secret_basic' is the default.
        -- 'none', 'client_secret_post',
    token_endpoint_auth_method VARCHAR(2048) NOT NULL DEFAULT 'client_secret_basic',
        -- 'client_secret_post', 'client_secret_basic',
        -- 'client_secret_jwt', 'private_key_jwt', and 'none'
    token_endpoint_auth_signing_alg VARCHAR(32) NOT NULL,
        -- JWS alg algorithm that MUST be used for signing the JWT used to authenticate
        --     the Client at the Token Endpoint for the 'private_key_jwt' and
        --     'client_secret_jwt' authentication methods.
        -- All Token Requests using these authentication methods from this
        --     Client MUST be rejected, if the JWT is not signed with this algorithm.
        -- Servers SHOULD support RS256. Indus supports RS256.
    default_max_age INTEGER DEFAULT 0, -- indicates there's no preference.
        -- If omitted, no default Maximum Authentication Age is specified.
        -- End-User MUST be actively authenticated if the End-User was authenticated
        --     longer ago than the specified number of seconds.
    require_auth_time TINYINT DEFAULT 0, -- indicates there's no preference.
        -- It is REQUIRED when the value is true. If this is false, the auth_time
        --    Claim can still be dynamically requested as an individual Claim for
        --    the ID Token using the claims request parameter described in
        --    Section 5.5.1 of Open
    default_acr_values TEXT CHARACTER SET ascii,
        -- The acr_values_supported discovery element contains a list of the
        --     supported acr values supported by this server. Values specified in
        --     the acr_values request parameter or an individual acr Claim request
        --      override these default values.
    initiate_login_uri VARCHAR(2048) CHARACTER SET ascii DEFAULT '',
        -- URI using the 'https' scheme that a third party can use to initiate
        --     a login by the RP, as specified in Section 4 of OpenID Connect Core 1.0.
    request_uris TEXT CHARACTER SET ascii,
        -- Array of request_uri values that are pre-registered by the RP for use at the OP.
        -- OPs can require that request_uri values used be pre-registered with the
        --     'require_request_uri_registration' discovery parameter.

    backchannel_token_delivery_mode CHAR(8) CHARACTER SET ascii DEFAULT '',
        -- REQUIRED. One of the following values: poll, ping, or push.
        -- When using the ping or poll mode, the Client MUST include the CIBA grant type
        --     in the "grant_types" field.
        -- When using the ping or push mode, the Client MUST register a
        --     client notification endpoint.
        -- Clients intending to send signed authentication requests MUST register
        --     the signature algorithm that will be used.
    backchannel_client_notification_endpoint VARCHAR(2048) CHARACTER SET ascii,
        -- REQUIRED if the token delivery mode is set to ping or push.
        -- MUST be an HTTPS URL.
        -- This is the endpoint to which the OP will post a notification after a
        --     successful or failed end-user authentication. It .
    backchannel_authentication_request_signing_alg VARCHAR(22) CHARACTER SET ascii DEFAULT '',
        -- JWS alg value that the Client will use for signing authentication requests.
        -- When omitted, the Client will not send signed authentication requests.
    backchannel_user_code_parameter BOOLEAN DEFAULT FALSE,
        -- OPTIONAL. specifies whether the Client supports the user_code parameter.
        -- If omitted, the default value is false.
        -- matces with 'backchannel_user_code_parameter_supported' OP parameter.

    audience TEXT,

    allowed_cors_origins TEXT,
    frontchannel_logout_uri TEXT NULL,
    frontchannel_logout_session_required BOOL NOT NULL DEFAULT FALSE,
    post_logout_redirect_uris TEXT NULL,
    backchannel_logout_uri TEXT NULL,
    backchannel_logout_session_required BOOL NOT NULL DEFAULT FALSE,

    dct TIMESTAMP NOT NULL,
        -- client desc creation timestamp.
    rgt TIMESTAMP,
        -- client registration timestamp.
    hct TIMESTAMP,
        -- last health checked/updated timestamp.
    lmt TIMESTAMP
        -- last modification timestamp.
);

CREATE UNIQUE INDEX index_client_desc_client_id ON client_desc(client_id);
