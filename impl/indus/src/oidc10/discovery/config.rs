
use serde::{ser, de};
use url::{Url};
use std::path::Path;
use std::{cmp, fmt};
use std::fmt::{Display, Error};

/// OpenID Connect Discovery 1.0 incorporating errata set 1
/// https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata
/// OAuth 2.0 Multiple Response Type Encoding Practices
/// https://openid.net/specs/oauth-v2-multiple-response-types-1_0.html
/// OAuth Parameters
/// https://www.iana.org/assignments/oauth-parameters/oauth-parameters.xhtml#client-metadata

pub enum ProviderConfigError {
    BadIssuerUrlScheme, BadIssuerUrlQuery(String), BadIssuerUrlFragment(String),
    BadAuthoizationEndpoinUrl(String),
    BadTokenEndpointUrl(String),
    BadUserinforEndpoint(String),
    BadJwksUrl(String),
    BadRegistrationEndpoint(String),
    MissingScopes, MissingOpenIdScope,
    MissingResponseTypes, MissingCodeResponseType,
    MissingIdTokenResponseType, MissingTokenIdTokenResponseType,
    MissingSubjectTypes,
    MissingSupportedIdTokenSigningAlgValues,
    BadSource(serde_json::Error),
    Custom(String),
}

impl std::fmt::Display for ProviderConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProviderConfigError::Custom(msg) => formatter.write_str(msg),
            _ => formatter.write_str("TODO: make error string!"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, std::fmt::Debug)]
pub struct ProviderConfig {
    issuer: Url,
    authorization_endpoint: Url,
    token_endpoint: Url,
    userinfo_endpoint: Url,
    jwks_uri: Url,
    registration_endpoint: Url,
    scopes_supported: Vec<String>,
    response_types_supported: Vec<String>,
    response_modes_supported: Vec<String>,
    grant_types_supported: Vec<String>,
    acr_values_supported: Vec<String>,
    subject_types_supported: Vec<String>,
    id_token_signing_alg_values_supported: Vec<String>,
    service_documentation: Url,
    claim_types_supported: Vec<String>, //[normal, aggregated, distributed.]
    claims_supported: Vec<String>,
    claims_locales_supported: Option<bool>,
    ui_locales_supported: Option<bool>,
    claims_parameter_supported: Option<bool>,
    request_parameter_supported: bool,
    request_uri_parameter_supported: bool,
    require_request_uri_registration: bool,
    op_policy_uri: Url,
    op_tos_uri: Url,
}

// alias type
type ConfigResult<T> = Result<T, ProviderConfigError>;

impl ProviderConfig {
    pub fn from_json(json: &str) -> ConfigResult<ProviderConfig> {
        let res: std::result::Result<ProviderConfig, serde_json::Error> = serde_json::from_str(json);
        match res {
            Ok(r) => {
                r.validate()
            },
            Err(e) => {
                log::error!("ProviderConfig::deserialize {:?}", e);
                Err(ProviderConfigError::BadSource(e))
            }
        }
    }
    pub fn from_json_file(path: &Path) -> ConfigResult<ProviderConfig> {
        println!("ProviderConfig::json file name {:?}", path);
        Err(ProviderConfigError::Custom("TODO: from_json_file".to_owned()))
    }
    pub fn from_sqlite3(path: &Path) -> ConfigResult<ProviderConfig> {
        Err(ProviderConfigError::Custom("TODO: from_sqlite3".to_owned()))
    }
    // https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata
    // 3.  OpenID Provider Metadata
    pub fn validate(self) -> std::result::Result<ProviderConfig, ProviderConfigError> {
        self.https_url(&self.issuer)?;
        self.https_query_url(&self.authorization_endpoint)?;
        self.https_query_url(&self.token_endpoint)?;
        self.https_port_path_url(&self.userinfo_endpoint)?;
        self.https_url(&self.jwks_uri)?;
        self.https_url(&self.registration_endpoint)?;
        Ok(self)
    }

    // https only scheme. no query component. no fragment. no port.
    pub fn https_url(&self, url : &Url) -> ConfigResult<&ProviderConfig> {
        if !url.scheme().eq_ignore_ascii_case("https") {
            Err(ProviderConfigError::BadIssuerUrlScheme)?
        }
        if url.host_str().is_none() || !url.username().eq("") ||
            url.password().is_some() || url.port().is_some() {
            Err(ProviderConfigError::BadIssuerUrlScheme)?
        }
        if url.query().is_some() {
            Err(ProviderConfigError::BadIssuerUrlQuery(url.to_string()))?
        }
        if url.fragment().is_some() {
            Err(ProviderConfigError::BadIssuerUrlFragment(url.to_string()))?
        }
        Ok(self)
    }

    // https only scheme. allow query component. no fragment. no user name and password. no port.
    pub fn https_query_url(&self, url: &Url) -> ConfigResult<&ProviderConfig> {
        if !url.scheme().eq_ignore_ascii_case("https") {
            Err(ProviderConfigError::BadIssuerUrlScheme)?
        }
        if url.host_str().is_none() || !url.username().eq("") || url.password().is_some() || url.port().is_some() {
            Err(ProviderConfigError::BadIssuerUrlScheme)?
        }
        if url.fragment().is_some() {
            Err(ProviderConfigError::BadIssuerUrlFragment(self.issuer.to_string()))?
        }
        Ok(self)
    }
    // https scheme. allow query. allow port. no fragment. no user name and password.
    pub fn https_port_path_url(&self, url: &Url) -> ConfigResult<&ProviderConfig> {
        if !url.scheme().eq_ignore_ascii_case("https") {
            Err(ProviderConfigError::BadIssuerUrlScheme)?
        }
        if url.host_str().is_none() || !url.username().eq("") || url.password().is_some() {
            Err(ProviderConfigError::BadIssuerUrlScheme)?
        }
        if url.fragment().is_some() {
            Err(ProviderConfigError::BadIssuerUrlFragment(self.issuer.to_string()))?
        }
        Ok(self)
    }
    // https scheme. no query. no fragment. no user name and password. allow port.
    pub fn vanilla_https_url(&self, url: &Url) -> ConfigResult<&ProviderConfig> {
        if !url.scheme().eq_ignore_ascii_case("https") {
            Err(ProviderConfigError::BadIssuerUrlScheme)?
        }
        Ok(self)
    }
}
