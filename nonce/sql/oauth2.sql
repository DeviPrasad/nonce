USE DATABASE indus_identity;

DROP TABLE IF EXISTS oauth2_access;
DROP TABLE IF EXISTS oauth2_pkce;
DROP TABLE IF EXISTS oauth2_refresh;
DROP TABLE IF EXISTS oauth2_code;

CREATE TABLE IF NOT EXISTS oauth2_access (
    txid          BINARY(20) NOT NULL,
    client_id     BINARY(16) NOT NULL,
    redirect_uri  TEXT, -- optional
    scope         TEXT, -- optional
    state         VARBINARY(2048), -- recommended
    granted_scope TEXT,
    form_data     TEXT,
    session_data  TEXT,
    reqat         TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX index_oauth2_access_txid ON oauth2_access (txid);
CREATE INDEX index_oauth2_access_client_id ON oauth2_access (client_id);

CREATE TABLE IF NOT EXISTS oauth2_pkce (
    txid           BINARY(20) NOT NULL,
    client_id      BINARY(16) NOT NULL,
    redirect_uri   TEXT,
    scope          TEXT,
    state          VARBINARY(2048),
    code_challenge VARBINARY(2048),
    code_challenge_method VARCHAR(16),
    granted_scope  TEXT,
    form_data      TEXT,
    session_data   TEXT,
    reqat          TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX index_oauth2_pkce_txid ON oauth2_pkce (txid);
CREATE INDEX index_oauth2_pkce_client_id ON oauth2_pkce (client_id);

CREATE TABLE IF NOT EXISTS oauth2_refresh (
    txid           BINARY(20) NOT NULL,
    client_id      BINARY(16) NOT NULL,
    scope          TEXT,
    code_challenge VARBINARY(2048),
    code_challenge_method VARCHAR(16),
    granted_scope  TEXT,
    form_data      TEXT,
    session_data   TEXT,
    reqat          TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX index_oauth2_refresh_txid ON oauth2_refresh (txid);
CREATE INDEX index_oauth2_refresh_client_id ON oauth2_refresh (client_id);

CREATE TABLE IF NOT EXISTS oauth2_code (
    txid           BINARY(20) NOT NULL,
    client_id      BINARY(16) NOT NULL,
    scope          TEXT,
    granted_scope  TEXT,
    code_challenge VARBINARY(2048),
    code_challenge_method VARCHAR(16),
    form_data      TEXT,
    session_data   TEXT,
    requested_at   TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX index_oauth2_code_txid ON oauth2_code (txid);
CREATE INDEX index_oauth2_code_client_id ON oauth2_code (client_id);
