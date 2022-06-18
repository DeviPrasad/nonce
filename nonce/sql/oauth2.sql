USE DATABASE idem;

DROP TABLE IF EXISTS oauth2_access;
DROP TABLE IF EXISTS oauth2_pkce;
DROP TABLE IF EXISTS oauth2_refresh;
DROP TABLE IF EXISTS oauth2_code;

CREATE TABLE IF NOT EXISTS oauth2_access (
    client_id     VARCHAR(512) NOT NULL,
    redirect_uri  VARCHAR(2048) CHARACTER SET ascii NOT NULL, -- optional
    scope         TEXT, -- optional
    state         VARBINARY(2048), -- recommended
    resource      TEXT,
    granted_scope TEXT,
    form_data     TEXT,
    session_data  TEXT,
    clock         TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX index_oauth2_access_txid ON oauth2_access (txid);
CREATE INDEX index_oauth2_access_client_id ON oauth2_access (client_id);

CREATE TABLE IF NOT EXISTS pkce (
    client_id      VARCHAR(512) NOT NULL,
    redirect_uri   VARCHAR(2048) CHARACTER SET ascii NOT NULL, -- optional
    scope          TEXT,
    state          VARBINARY(2048),
    resource       TEXT,
    code_challenge VARBINARY(2048) DEFAULT NULL,
    code_challenge_method VARCHAR(16) DEFAULT NULL,
    granted_scope  TEXT,
    form_data      TEXT,
    session_data   TEXT,
    clock          TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX index_oauth2_pkce_txid ON oauth2_pkce (txid);
CREATE INDEX index_oauth2_pkce_client_id ON oauth2_pkce (client_id);

CREATE TABLE IF NOT EXISTS oauth2_refresh (
    client_id      VARCHAR(512) NOT NULL,
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

CREATE TABLE IF NOT EXISTS code (
    client_id      VARCHAR(512) NOT NULL,
    code           TEXT,
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
