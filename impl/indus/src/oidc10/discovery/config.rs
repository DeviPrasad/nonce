
use std::path::Path;
use serde::{Serialize, Deserialize};
use url::{Url, ParseError};

/// OpenID Connect Discovery 1.0 incorporating errata set 1
/// https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata
/// OAuth 2.0 Multiple Response Type Encoding Practices
/// https://openid.net/specs/oauth-v2-multiple-response-types-1_0.html

pub enum ProviderConfigError {
    MissingIssuerUrl, BadIssuerUrl(String),
    MissingAuthorizationEndpointUrl, BadAuthoizationEndpoinUrl(String),
    MissingTokenEndpointUrl, BadTokenEndpointUrl(String),
    MissingUserinforEndpoint, BadUserinforEndpoint(String),
    MissingJwksUrl, BadJwksUrl(String),
    MissingRegistrationEndpoint, BadRegistrationEndpoint(String),
    MissingScopes, MissingOpenIdScope,
    MissingResponseTypes, MissingCodeResponseType,
    MissingIdTokenResponseType, MissingTokenIdTokenResponseType,
    MissingSubjectTypes,
    MissingSupportedIdTokenSigningAlgValues,
    BadSource,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ProviderConfig {
    issuer: Url,
    authorization_endpoint: Option<Url>,
    token_endpoint: Option<Url>,
    userinfo_endpoint: Option<Url>,
    jwks_uri: Option<Url>,
    registration_endpoint: Option<Url>,
    scopes_supported: Vec<String>,
    response_types_supported: Vec<String>,
    response_modes_supported: Vec<String>,
    grant_types_supported: Vec<String>,
    acr_values_supported: Vec<String>,
    subject_types_supported: Vec<String>,
    id_token_signing_alg_values_supported: Vec<String>,
}

impl ProviderConfig {
    pub fn from_json(json: &str) -> Result<ProviderConfig, ProviderConfigError> {
        println!("ProviderConfig::serialize {:?}", json);
        let res: Result<ProviderConfig, serde_json::Error> = serde_json::from_str(json);
        match res {
            Ok(r) => {
                println!("ProviderConfig::serialize {:?}", r);
                Ok(r)
            },
            Err(e) => {
                println!("ProviderConfig::serialize {:?}", e);
                Err(ProviderConfigError::BadSource)
            }
        }
    }
    pub fn from_json_file(path: &Path) -> Result<ProviderConfig, ProviderConfigError> {
        println!("ProviderConfig::json file name {:?}", path);
        Err(ProviderConfigError::BadSource)
    }
    pub fn from_sqlite3(path: &Path) -> Result<ProviderConfig, ProviderConfigError> {
        Err(ProviderConfigError::BadSource)
    }
}

pub fn test() {
    let indus_basic = r#"
        {
            "issuer": "https://www.indus.in/nonce/",
            "authorization_endpoint": "https://www.indus.in/nonce/auth",
            "token_endpoint": "https://www.indus.in/nonce/token",
            "userinfo_endpoint": "https://www.indus.in/nonce/userinfo",
            "jwks_uri": "https://www.indus.in/nonce/oidc/jwks",
            "registration_endpoint": "https://www.indus.in/nonce/oidc/registration",
            "scopes_supported": ["openid"],
            "response_types_supported": ["code", "id_token", "token id_token"],
            "response_modes_supported": ["query", "fragment"],
            "grant_types_supported": ["authorization_code", "implicit"],
            "acr_values_supported": [],
            "subject_types_supported": ["public", "public"],
            "id_token_signing_alg_values_supported": []
        }"#;
    let res = ProviderConfig::from_json(indus_basic);
    assert!(res.is_ok());
}
