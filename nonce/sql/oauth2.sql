
CREATE TABLE IF NOT EXISTS nonce_oauth2_access (
	signature      	varchar(255) NOT NULL PRIMARY KEY,
	request_id  	varchar(40) NOT NULL,
	requested_at    timestamp NOT NULL DEFAULT now(),
	client_id  		varchar(255) NOT NULL
	scope  			text NOT NULL,
	granted_scope 	text NOT NULL,
	form_data  		text NOT NULL,
	session_data  	text NOT NULL,
    subject         varchar(255),
    active          BOOL NOT NULL DEFAULT TRUE,
    challenge_id    varchar(40) NULL,

    CREATE UNIQUE INDEX nonce_oauth2_access_request_id_index ON nonce_oauth2_access (request_id),
    CREATE INDEX nonce_oauth2_access_client_id_index ON nonce_oauth2_access (client_id),
    CREATE INDEX nonce_oauth2_access_challenge_id_index ON nonce_oauth2_access (challenge_id)
);

CREATE TABLE IF NOT EXISTS nonce_oauth2_refresh (
	signature      	varchar(255) NOT NULL PRIMARY KEY,
	request_id  	varchar(40) NOT NULL,
	requested_at  	timestamp NOT NULL DEFAULT now(),
	client_id  		varchar(255) NOT NULL
	scope  			text NOT NULL,
	granted_scope 	text NOT NULL,
	form_data  		text NOT NULL,
	session_data  	text NOT NULL,
    subject         varchar(255),
    active          BOOL NOT NULL DEFAULT TRUE,
    challenge_id    varchar(40) NULL,
    CREATE UNIQUE INDEX nonce_oauth2_refresh_request_id_index ON nonce_oauth2_refresh (request_id),
    CREATE INDEX nonce_oauth2_refresh_client_id_index ON nonce_oauth2_refresh (client_id),
    CREATE INDEX nonce_oauth2_refresh_challenge_id_index ON nonce_oauth2_refresh (challenge_id)
);

CREATE TABLE IF NOT EXISTS nonce_oauth2_code (
	signature      	varchar(255) NOT NULL PRIMARY KEY,
	request_id  	varchar(40) NOT NULL,
	requested_at  	timestamp NOT NULL DEFAULT now(),
	client_id  		varchar(255) NOT NULL
	scope  			text NOT NULL,
	granted_scope 	text NOT NULL,
	form_data  		text NOT NULL,
	session_data  	text NOT NULL,
    subject         varchar(255),
    active          BOOL NOT NULL DEFAULT TRUE,
    challenge_id    varchar(40) NULL,
    CREATE INDEX nonce_oauth2_code_client_id_index ON nonce_oauth2_code (client_id),
    CREATE INDEX nonce_oauth2_code_challenge_id_index ON nonce_oauth2_code (challenge_id)
);

CREATE TABLE IF NOT EXISTS nonce_oauth2_oidc (
	signature      	varchar(255) NOT NULL PRIMARY KEY,
	request_id  	varchar(40) NOT NULL,
	requested_at  	timestamp NOT NULL DEFAULT now(),
	client_id  		varchar(255) NOT NULL
	scope  			text NOT NULL,
	granted_scope 	text NOT NULL,
	form_data  		text NOT NULL,
	session_data  	text NOT NULL,
    subject         varchar(255),
    active          BOOL NOT NULL DEFAULT TRUE,
    challenge_id    varchar(40) NULL,
    CREATE INDEX nonce_oauth2_oidc_client_id_index ON nonce_oauth2_oidc (client_id),
    CREATE INDEX nonce_oauth2_oidc_challenge_id_index ON nonce_oauth2_oidc (challenge_id)
);

CREATE TABLE IF NOT EXISTS nonce_oauth2_pkce (
	signature      	varchar(255) NOT NULL PRIMARY KEY,
	request_id  	varchar(40) NOT NULL,
	requested_at  	timestamp NOT NULL DEFAULT now(),
	client_id  		varchar(255) NOT NULL
	scope  			text NOT NULL,
	granted_scope 	text NOT NULL,
	form_data  		text NOT NULL,
	session_data  	text NOT NULL,
	subject 		varchar(255) NOT NULL,
    active          BOOL NOT NULL DEFAULT TRUE,
    challenge_id    varchar(40) NUL
    CREATE INDEX nonce_oauth2_pkce_client_id_index ON nonce_oauth2_pkce (client_id),
    CREATE INDEX nonce_oauth2_pkce_challenge_id_index ON nonce_oauth2_pkce (challenge_id)

);

REATE TABLE IF NOT EXISTS nonce_oauth2_jti_blacklist (
	signature       varchar(64) NOT NULL PRIMARY KEY,
	expires_at  	timestamp NOT NULL DEFAULT now(),
    CREATE INDEX nonce_oauth2_jti_blacklist_expiry_index ON nonce_oauth2_jti_blacklist (expires_at)
);
