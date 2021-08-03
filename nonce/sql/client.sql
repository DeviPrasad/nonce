


CREATE TABLE IF NOT EXISTS nonce_client (
    pk                INTEGER UNSIGNED AUTO_INCREMENT PRIMARY KEY;
    -- PGSQL pk                SERIAL;
	id      	      varchar(255) NOT NULL,
	client_name  	  text NOT NULL,
	client_secret  	  text NOT NULL,
	redirect_uris  	  text NOT NULL,
	grant_types  	  text NOT NULL,
	response_types    text NOT NULL,
	scope  			  text NOT NULL,
	owner  			  text NOT NULL,
	policy_uri  	  text NOT NULL,
	tos_uri  		  text NOT NULL,
	client_uri  	  text NOT NULL,
	logo_uri  		  text NOT NULL,
	contacts  		  text NOT NULL,
	confidential      boolean NOT NULL,
    -- auth_method='none' when confidential=false
    token_endpoint_auth_method   VARCHAR(32) NOT NULL,
    -- UPDATE hydra_client SET token_endpoint_auth_signing_alg = 'RS256' WHERE token_endpoint_auth_method = 'private_key_jwt';
    token_endpoint_auth_signing_alg VARCHAR(32) NOT NULL,
    client_secret_expires_at     INTEGER NOT NULL DEFAULT 0,
    sector_identifier_uri  TEXT;,
    jwks                 TEXT,
    jwks_uri             TEXT,
    request_uris         TEXT,
    audience             TEXT,
    subject_type         VARCHAR(15) NOT NULL DEFAULT '',
    allowed_cors_origins TEXT,
    request_object_signing_alg   VARCHAR(10) NOT NULL,
    userinfo_signed_response_alg VARCHAR(10) NOT NULL,

    frontchannel_logout_uri TEXT NULL;
    frontchannel_logout_session_required BOOL NOT NULL DEFAULT FALSE;
    post_logout_redirect_uris TEXT NULL;
    backchannel_logout_uri TEXT NULL;
    backchannel_logout_session_required BOOL NOT NULL DEFAULT FALSE;

    additional_data TEXT NOT NULL DEFAULT "{}",

    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now(),

    CREATE UNIQUE INDEX nonce_client_index_id ON nonce_client (id);
);

-- +migrate Down
DROP TABLE IF EXISTS hydra_client;
