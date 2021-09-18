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
#[derive(std::fmt::Debug)]
pub enum ProviderConfigError {
    BadIssuerUrlScheme, BadIssuerUrlQuery(String), BadIssuerUrlFragment(String),
    BadAuthoizationEndpoinUrl(String),
    BadTokenEndpointUrl(String),
    BadUserinforEndpoint(String),
    BadJwksUrl(String),
    BadRegistrationEndpoint(String),

    EmptySupportedScopes, MissingOpenIdScope,

    EmptyResponseTypes, BadResponseType,

    EmptyResponseModes, BadResponseMode,

    EmptyGrantTypes, BadGrantType,

    BadAcrValue,

    MissingIdTokenResponseType, MissingTokenIdTokenResponseType,

    EmptySubjectTypes, BadSubjectType,

    EmptyIdTokenSigningAlgValue, BadIdTokenSigningAlgValue, IdTokenSigningAlgNoneNotSupported,
    EmptyIdTokenEncryptionAlgValue, BadIdTokenEncryptionAlgValue,
    EmptyIdTokenEncryptionEncValue, BadIdTokenEncryptionEncValue,

    EmptyUserInfoSigningAlgValue, BadUserInfoSigningAlgValue,
    EmptyUserInfoEncryptionAlgValue, BadUserInfoEncryptionAlgValue,
    EmptyUserInfoEncryptionEncValue, BadUserInfoEncryptionEncValue,

    EmptyRequestObjectSigningAlgValue, BadRequestObjectSigningAlgValue,
    EmptyRequestObjectEncryptionAlgValue, BadRequestObjectEncryptionAlgValue,
    EmptyRequestObjectEncryptionEncValue, BadRequestObjectEncryptionEncValue,

    EmptyTokenEndpointAuthMethodsValue, BadTokenEndpointAuthMethodValue,

    EmptyTokenEndpointAuthSigningAlgValue, BadTokenEndpointAuthSigningAlgValue,
    MissingMandatoryTokenEndpointAuthSigningAlgRS256, BadTokenEndpointAuthSigningAlgNone,

    EmptyConsentDisplayValue, BadConsentDisplayValue, OnlyNoneConsentDisplayValue,

    BadClaimTypeValue,
    EmptyClaimNames, BadClaimName,

    BadRequestParameter, BadRequestUriParameter, BadRequireRequestUriRegistration,

    BadSource(serde_json::Error),
    Custom(String),
}

const INDUS_SCOPE_NAME_AADHAR_ID: &str = "indus.aadharId";
const INDUS_SCOPE_NAME_VOTER_ID: &str = "indus.voterId";
const INDUS_SCOPE_NAME_PAN: &str = "indus.pan";
const OIDC_SCOPE_NAME_OPENID: &str = "openid";

// https://openid.net/specs/openid-connect-core-1_0.html
// OpenID Connect Core 1.0.
// 15.2 Mandatory to Implement Features for Dynamic OpenID Providers.
// https://openid.net/specs/oauth-v2-multiple-response-types-1_0.html
// OAuth 2.0 Multiple Response Type Encoding Practices
// 5. Definitions of Multiple-Valued Response Type Combinations.
const INDUS_RESPONSE_TYPES: [&str; 8] = [
    "code",           // Mandatory for Dynamic OpenID Providers.
    "id_token",       // Mandatory for Dynamic OpenID Providers.
    "id_token token", // Mandatory for Dynamic OpenID Providers.
    "token",
    "code token",
    "code id_token",
    "code id_token token",
    "none",
];

// https://openid.net/specs/oauth-v2-multiple-response-types-1_0.html
// OAuth 2.0 Multiple Response Type Encoding Practices
// 2.1.  Response Modes
const INDUS_RESPONSE_MODES: [&str; 2] = ["fragment", "query"];

//OAuth 2.0 Dynamic Registration - https://www.rfc-editor.org/rfc/rfc7591.html - grant_types.
const INDUS_GRANT_TYPES: [&str; 9] = [
    "authorization_code", "client_credentials", "implicit", "password", "refresh_token",
    //JWT Bearer Token Grant Type Profile for OAuth 2.0
    "urn:ietf:params:oauth:grant-type:jwt-bearer",
    //SAML 2.0 Bearer Assertion Grant Type Profile for OAuth 2.0
    "urn:ietf:params:oauth:grant-type:saml2-bearer",
    //Device flow grant type for OAuth 2.0 - [RFC8628, Section 3.4]
    "urn:ietf:params:oauth:grant-type:device_code",
    //Token exchange grant type for OAuth 2.0 - RFC 8693 OAuth 2.0 Token Exchange
    "urn:ietf:params:oauth:grant-type:token-exchange",
];

// https://openid.net/specs/openid-connect-core-1_0.html
// OpenID Connect Core 1.0.
// 8. Subject Identifier Types.
const INDUS_SUBJECT_TYPES: [&str; 2] = ["public", "pairwise"];

// https://datatracker.ietf.org/doc/html/rfc7518
// JSON Web Algorithms (JWA).
// 3. Cryptographic Algorithms for Digital Signatures and MACs.
// 3.1 "alg" (Algorithm) Header Parameter Values for JWS.
const INDUS_SIGNING_ALGORITHMS: [&str; 7] = [
    "HS256", // Required
    "HS512", // optional
    "RS256", // Recommended - RSASSA-PKCS1-v1_5 using SHA-256
    "RS512", // optional - RSASSA-PKCS1-v1_5 using SHA-512
    "ES256", // Recommended+ - ECDSA using P-256 and SHA-256
    "ES512", // optional - ECDSA using P-521 and SHA-512
    "none",  // well, well!
];

// https://datatracker.ietf.org/doc/html/rfc7518
// JSON Web Algorithms (JWA).
// 4. Cryptographic Algorithms for Key Management.
// 4.1 "alg" (Algorithm) Header Parameter Values for JWE.
const INDUS_CEK_ENCRYPTION_ALGORITHMS: [&str; 11] = [
    "RSA1_5",         //NOT Recommended by JWT BCP RFC 8725. RSAES-PKCS1-v1_5 with 2048-bit keys.
    "RSA-OAEP",       //RSAES OAEP using default parameters.
    "RSA-OAEP-256",   //RSAES OAEP using SHA-256 and MGF1 with SHA-256.
    "A128KW",         //Recommened. AES Key Wrap with default initial value using 128-bit key.
    "A256KW",         //Recommened. AES Key Wrap with default initial value using 256-bit key.
    "ECDH-ES",        //Recommened. ECDH Ephemeral Static Key agreement.
    "ECDH-ES+A128KW", //Recommened. ECDH-ES using Concat KDF and CEK wrapped with "A128KW"
    "ECDH-ES+A256KW", //Recommened. ECDH-ES using Concat KDF and CEK wrapped with "A256KW"
    "A128GCMKW",      // Key wrapping with AES GCM using 128-bit key
    "A256GCMKW",      // Key wrapping with AES GCM using 256-bit key
    "dir" // Direct use of a shared symmetric key as the CEK
];

// https://datatracker.ietf.org/doc/html/rfc7518
// JSON Web Algorithms (JWA).
// 5. Cryptographic Algorithms for Content Encryption.
// 5.1 "enc" (Encryption Algorithm) Header Parameter Values for JWE.
const INDUS_CONTENT_ENCRYPTION_ALGORITHMS: [&str; 4] = [
    "A128CBC-HS256",  // Required. AES_128_CBC_HMAC_SHA_256 authenticated encryption.
    "A256CBC-HS512",  // Required. AES_256_CBC_HMAC_SHA_512 authenticated encryption.
    "A128GCM",        // Recommened.
    "A256GCM",        // Recommened.
];

// https://openid.net/specs/openid-connect-core-1_0.html
// OpenID Connect Core 1.0.
// 9. Client Authentication.
const INDUS_TOKEN_ENDPOINT_AUTH_MATHODS: [&str; 4] = [
    "client_secret_post",
    "client_secret_basic",
    "client_secret_jwt",
    "private_key_jwt"
];

// https://openid.net/specs/openid-connect-core-1_0.html
// OpenID Connect Core 1.0.
// 3.1.2.1.  Authentication Request
const INDUS_ENDUSER_CONSENT_DISPLAY_VALUES: [&str; 4] = [
    "none", //Authorization Server MUST NOT display any authentication or consent user interface pages.
    "login", //The Authorization Server SHOULD prompt the End-User for reauthentication.
    "consent", // The Authorization Server SHOULD prompt the End-User for consent before returning information to RP.
    "select_account" //he Authorization Server SHOULD prompt the End-User to select a user account.
];

// https://openid.net/specs/openid-connect-core-1_0.html
// OpenID Connect Core 1.0.
// 5.6 Claim Types.
const INDUS_CLAIM_TYPES: [&str; 3] = [ "normal", "aggregated", "distributed" ];

// https://openid.net/specs/openid-connect-core-1_0.html#AuthRequest
// OpenID Connect Core 1.0.
// 5. Claims - 5.1 Standard Claims.
const INDUS_CLAIM_NAMES:[&str; 25] = [
    "sub", "name", "given_name", "family_name", "middle_name", "nickname", "preferred_username",
    "profile", "picture", "website", "email", "email_verified", "gender", "birthdate",
    "zoneinfo", "locale", "phone_number", "phone_number_verified", "address", "updated_at",
    "aadhaar", "voterid", "pan", "indus_id", "indus_key"
];

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
    acr_values_supported: Option<Vec<String>>,
    subject_types_supported: Vec<String>,
    id_token_signing_alg_values_supported: Vec<String>,
    id_token_encryption_alg_values_supported: Option<Vec<String>>,
    id_token_encryption_enc_values_supported: Option<Vec<String>>,
    userinfo_signing_alg_values_supported: Option<Vec<String>>,
    userinfo_encryption_alg_values_supported: Option<Vec<String>>,
    userinfo_encryption_enc_values_supported: Option<Vec<String>>,
    request_object_signing_alg_values_supported: Option<Vec<String>>,
    request_object_encryption_alg_values_supported: Option<Vec<String>>,
    request_object_encryption_enc_values_supported: Option<Vec<String>>,
    token_endpoint_auth_methods_supported: Vec<String>,
    token_endpoint_auth_signing_alg_values_supported: Option<Vec<String>>,
    display_values_supported: Vec<String>,
    claim_types_supported: Option<Vec<String>>,
    claims_supported: Vec<String>,
    service_documentation: Url,
    claims_locales_supported: Option<bool>, // NOT available as yet (2021-SEPT-10).
    ui_locales_supported: Option<bool>,     // NOT available as yet (2021-SEPT-10).
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
    pub fn have_scope_indus_aadhaarid(&self) -> bool {
        self.scopes_supported.iter().any(|scope| scope == INDUS_SCOPE_NAME_AADHAR_ID)
    }
    pub fn have_scope_indus_voterid(&self) -> bool {
        self.scopes_supported.iter().any(|scope| scope == INDUS_SCOPE_NAME_VOTER_ID)
    }
    pub fn have_scope_indus_pan(&self) -> bool {
        self.scopes_supported.iter().any(|scope| scope == INDUS_SCOPE_NAME_PAN)
    }
    pub fn from_json(json: &str) -> ConfigResult<ProviderConfig> {
        let res: std::result::Result<ProviderConfig, serde_json::Error> = serde_json::from_str(json);
        match res {
            Ok(mut r) => {
                r.trim();
                r.validate()
            },
            Err(e) => {
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
    fn trim(self: &mut ProviderConfig) {
        self.response_types_supported.sort();
        self.response_types_supported.dedup();
        self.response_modes_supported.sort();
        self.response_modes_supported.dedup();
        self.grant_types_supported.sort();
        self.grant_types_supported.dedup();
        self.subject_types_supported.sort();
        self.subject_types_supported.dedup();
        self.id_token_signing_alg_values_supported.sort();
        self.id_token_signing_alg_values_supported.dedup();
        if self.id_token_encryption_alg_values_supported.is_some() {
            let algref = self.id_token_encryption_alg_values_supported.as_mut().unwrap();
            algref.sort();
            algref.dedup();
        }
        if self.id_token_encryption_enc_values_supported.is_some() {
            let algref = self.id_token_encryption_enc_values_supported.as_mut().unwrap();
            algref.sort();
            algref.dedup();
        }
        if self.userinfo_signing_alg_values_supported.is_some() {
            let algref = self.userinfo_signing_alg_values_supported.as_mut().unwrap();
            algref.sort();
            algref.dedup();
        }
        if self.userinfo_encryption_alg_values_supported.is_some() {
            let algref = self.userinfo_encryption_alg_values_supported.as_mut().unwrap();
            algref.sort();
            algref.dedup();
        }
        if self.userinfo_encryption_enc_values_supported.is_some() {
            let algref = self.userinfo_encryption_enc_values_supported.as_mut().unwrap();
            algref.sort();
            algref.dedup();
        }
        if self.request_object_signing_alg_values_supported.is_some() {
            let algref = self.request_object_signing_alg_values_supported.as_mut().unwrap();
            algref.sort();
            algref.dedup();
        }
        if self.request_object_encryption_alg_values_supported.is_some() {
            let algref = self.request_object_encryption_alg_values_supported.as_mut().unwrap();
            algref.sort();
            algref.dedup();
        }
        if self.request_object_encryption_enc_values_supported.is_some() {
            let algref = self.request_object_encryption_enc_values_supported.as_mut().unwrap();
            algref.sort();
            algref.dedup();
        }
        self.token_endpoint_auth_methods_supported.sort();
        self.token_endpoint_auth_methods_supported.dedup();
        if self.token_endpoint_auth_signing_alg_values_supported.is_some() {
            let algref = self.token_endpoint_auth_signing_alg_values_supported.as_mut().unwrap();
            algref.sort();
            algref.dedup();
        }
        if self.claim_types_supported.is_none() {
            self.claim_types_supported = Some(vec!("normal".to_string()));
        } else if self.claim_types_supported.is_some() {
            let ctref = self.claim_types_supported.as_mut().unwrap();
            if ctref.len() == 0 {
                self.claim_types_supported = Some(vec!("normal".to_string()));
            } else {
                ctref.sort();
                ctref.dedup();
            }
        }
        self.display_values_supported.sort();
        self.display_values_supported.dedup();
        self.claims_supported.sort();
        self.claims_supported.dedup();
    }
    // https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata
    // 3.  OpenID Provider Metadata
    fn validate(self) -> std::result::Result<ProviderConfig, ProviderConfigError> {
        self.https_boring_url(&self.issuer)?;
        self.https_query_url(&self.authorization_endpoint)?;
        self.https_query_url(&self.token_endpoint)?;
        self.https_port_url(&self.userinfo_endpoint)?;
        self.https_port_query_url(&self.jwks_uri)?;
        self.https_boring_url(&self.registration_endpoint)?;
        self.have_openid_scope()?;
        self.have_indus_openid_response_types()?;
        self.have_indus_openid_response_modes()?;
        self.have_indus_grant_types()?;
        self.have_acr_values()?;
        self.have_subject_types()?;
        self.have_id_token_signing_alg_values()?;
        self.have_id_token_encryption_alg_values()?;
        self.have_id_token_encryption_enc_values()?;
        self.have_userinfo_signing_alg_values()?;
        self.have_userinfo_encryption_alg_values()?;
        self.have_userinfo_encryption_enc_values()?;
        self.have_request_object_signing_alg_values()?;
        self.have_request_object_encryption_alg_values()?;
        self.have_request_object_encryption_enc_values()?;
        self.have_token_endpoint_auth_methods_supported()?;
        self.have_token_endpoint_auth_signing_alg_values_supported()?;
        self.have_consent_display_values()?;
        self.have_claim_types()?;
        self.have_claim_names()?;
        self.https_port_query_url(&self.service_documentation)?;
        self.have_request_param_uri_registration_values()?;
        self.https_port_query_url(&self.op_policy_uri)?;
        self.https_port_query_url(&self.op_tos_uri)?;
        Ok(self)
    }
    // https scheme. no port. no user name. no password. no query. no fragment.
    pub fn https_boring_url(&self, url: &Url) -> ConfigResult<&ProviderConfig> {
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
    // https scheme. allow query. no port. no user name and password. no fragment.
    fn https_query_url(&self, url: &Url) -> ConfigResult<&ProviderConfig> {
        self.https_port_query_url(url)?;
        if url.port().is_some() {
            Err(ProviderConfigError::BadIssuerUrlScheme)?
        }
        Ok(self)
    }
    // https scheme. allow port. allow query. no user name and password. no fragment.
    pub fn https_port_url(&self, url: &Url) -> ConfigResult<&ProviderConfig> {
        self.https_port_query_url(url)
    }
    // https scheme. allow port. allow query. no user name. no password. no fragment.
    pub fn https_port_query_url(&self, url: &Url) -> ConfigResult<&ProviderConfig> {
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
    // must support 'openid'
    pub fn have_openid_scope(&self) -> ConfigResult<&ProviderConfig> {
        if self.scopes_supported.len() == 0 {
            Err(ProviderConfigError::EmptySupportedScopes)?
        }
        if !self.scopes_supported.iter().any(|scope| scope == OIDC_SCOPE_NAME_OPENID) {
            Err(ProviderConfigError::MissingOpenIdScope)?
        }
        Ok(self)
    }
    pub fn have_indus_openid_response_types(&self) -> ConfigResult<&ProviderConfig> {

        if self.response_types_supported.len() == 0 {
            Err(ProviderConfigError::EmptyResponseTypes)?
        }
        if !self.response_types_supported.iter().all(|rt| INDUS_RESPONSE_TYPES.contains(&rt.as_str())) {
            Err(ProviderConfigError::BadResponseType)?
        }
        Ok(self)
    }
    pub fn have_indus_openid_response_modes(&self) -> ConfigResult<&ProviderConfig> {
        if self.response_modes_supported.len() == 0 {
            Err(ProviderConfigError::EmptyResponseModes)?
        }
        if !self.response_modes_supported.iter().all(|rm| INDUS_RESPONSE_MODES.contains(&rm.as_str())) {
            Err(ProviderConfigError::BadResponseMode)?
        }
        Ok(self)
    }
    pub fn have_indus_grant_types(&self) -> ConfigResult<&ProviderConfig> {
        if self.grant_types_supported.len() == 0 {
            Err(ProviderConfigError::EmptyGrantTypes)?
        }
        if !self.grant_types_supported.iter().all(|gt| INDUS_GRANT_TYPES.contains(&gt.as_str())) {
            Err(ProviderConfigError::BadGrantType)?
        }
        Ok(self)
    }
    pub fn have_acr_values(&self) -> ConfigResult<&ProviderConfig> {
        let av = self.acr_values_supported.as_ref();
        if av.is_some() && av.unwrap().len() > 0 {
            Err(ProviderConfigError::BadAcrValue)?
        }
        Ok(self)
    }
    pub fn have_subject_types(&self) -> ConfigResult<&ProviderConfig> {
        if self.subject_types_supported.len() == 0 {
            Err(ProviderConfigError::EmptySubjectTypes)?
        }
        if !self.subject_types_supported.iter().all(|sub| INDUS_SUBJECT_TYPES.contains(&sub.as_str())) {
            Err(ProviderConfigError::BadSubjectType)?
        }
        Ok(self)
    }
    // Strict NO to id_token signing algorithm "none".
    // MUST have at least one signing algorithm.
    pub fn have_id_token_signing_alg_values(&self) -> ConfigResult<&ProviderConfig> {
        if self.id_token_signing_alg_values_supported.len() == 0 {
            Err(ProviderConfigError::EmptyIdTokenSigningAlgValue)?
        }
        if !self.id_token_signing_alg_values_supported.iter().all(|alg| INDUS_SIGNING_ALGORITHMS.contains(&alg.as_str())) {
            Err(ProviderConfigError::BadIdTokenSigningAlgValue)?
        }
        if self.id_token_signing_alg_values_supported.iter().any(|sub| sub.eq("none")) {
            Err(ProviderConfigError::IdTokenSigningAlgNoneNotSupported)?
        }
        Ok(self)
    }
    // id_token encryption is optional.
    pub fn have_id_token_encryption_alg_values(&self) -> ConfigResult<&ProviderConfig> {
        let algref = self.id_token_encryption_alg_values_supported.as_ref();
        if algref.is_some() {
            if algref.unwrap().len() == 0 { Err(ProviderConfigError::EmptyIdTokenEncryptionAlgValue)? }
            if !algref.unwrap().iter().all(|alg| INDUS_CEK_ENCRYPTION_ALGORITHMS.contains(&alg.as_str())) {
                Err(ProviderConfigError::BadIdTokenEncryptionAlgValue)?
            }
        }
        Ok(self)
    }
    pub fn have_id_token_encryption_enc_values(&self) -> ConfigResult<&ProviderConfig> {
        let algref = self.id_token_encryption_enc_values_supported.as_ref();
         if algref.is_some() {
             if algref.unwrap().len() == 0 { Err(ProviderConfigError::EmptyIdTokenEncryptionEncValue)? }
             if !algref.unwrap().iter().all(|alg| INDUS_CONTENT_ENCRYPTION_ALGORITHMS.contains(&alg.as_str())) {
                 Err(ProviderConfigError::BadIdTokenEncryptionEncValue)?
             }
        }
        Ok(self)
    }
    // userinfo signing is optional. "none" is allowed.
    pub fn have_userinfo_signing_alg_values(&self) -> ConfigResult<&ProviderConfig> {
        let algref = self.userinfo_signing_alg_values_supported.as_ref();
        if algref.is_some() {
            if algref.unwrap().len() == 0 {
                Err(ProviderConfigError::EmptyUserInfoSigningAlgValue)?
            }
            if !algref.unwrap().iter().all(|alg| INDUS_SIGNING_ALGORITHMS.contains(&alg.as_str())) {
                Err(ProviderConfigError::BadUserInfoSigningAlgValue)?
            }
        }
        Ok(self)
    }
    pub fn have_userinfo_encryption_alg_values(&self) -> ConfigResult<&ProviderConfig> {
        let algref = self.userinfo_encryption_alg_values_supported.as_ref();
        if algref.is_some() {
            if algref.unwrap().len() == 0 { Err(ProviderConfigError::EmptyUserInfoEncryptionAlgValue)? }
            if !algref.unwrap().iter().all(|alg| INDUS_CEK_ENCRYPTION_ALGORITHMS.contains(&alg.as_str())) {
                Err(ProviderConfigError::BadUserInfoEncryptionAlgValue)?
            }
        }
        Ok(self)
    }
    pub fn have_userinfo_encryption_enc_values(&self) -> ConfigResult<&ProviderConfig> {
        let algref = self.userinfo_encryption_enc_values_supported.as_ref();
         if algref.is_some() {
             if algref.unwrap().len() == 0 { Err(ProviderConfigError::EmptyUserInfoEncryptionEncValue)? }
             if !algref.unwrap().iter().all(|alg| INDUS_CONTENT_ENCRYPTION_ALGORITHMS.contains(&alg.as_str())) {
                 Err(ProviderConfigError::BadUserInfoEncryptionEncValue)?
             }
        }
        Ok(self)
    }
    // userinfo signing is optional. "none" is allowed.
    pub fn have_request_object_signing_alg_values(&self) -> ConfigResult<&ProviderConfig> {
        let algref = self.request_object_signing_alg_values_supported.as_ref();
        if algref.is_some() {
            if algref.unwrap().len() == 0 {
                Err(ProviderConfigError::EmptyRequestObjectSigningAlgValue)?
            }
            if !algref.unwrap().iter().all(|alg| INDUS_SIGNING_ALGORITHMS.contains(&alg.as_str())) {
                Err(ProviderConfigError::BadRequestObjectSigningAlgValue)?
            }
        }
        Ok(self)
    }
    pub fn have_request_object_encryption_alg_values(&self) -> ConfigResult<&ProviderConfig> {
        let algref = self.request_object_encryption_alg_values_supported.as_ref();
        if algref.is_some() {
            if algref.unwrap().len() == 0 { Err(ProviderConfigError::EmptyRequestObjectEncryptionAlgValue)? }
            if !algref.unwrap().iter().all(|alg| INDUS_CEK_ENCRYPTION_ALGORITHMS.contains(&alg.as_str())) {
                Err(ProviderConfigError::BadRequestObjectEncryptionAlgValue)?
            }
        }
        Ok(self)
    }
    pub fn have_request_object_encryption_enc_values(&self) -> ConfigResult<&ProviderConfig> {
        let algref = self.request_object_encryption_enc_values_supported.as_ref();
         if algref.is_some() {
             if algref.unwrap().len() == 0 { Err(ProviderConfigError::EmptyRequestObjectEncryptionEncValue)? }
             if !algref.unwrap().iter().all(|alg| INDUS_CONTENT_ENCRYPTION_ALGORITHMS.contains(&alg.as_str())) {
                 Err(ProviderConfigError::BadRequestObjectEncryptionEncValue)?
             }
        }
        Ok(self)
    }
    pub fn have_token_endpoint_auth_methods_supported(&self) -> ConfigResult<&ProviderConfig> {
        if self.token_endpoint_auth_methods_supported.len() == 0 {
             Err(ProviderConfigError::EmptyTokenEndpointAuthMethodsValue)?
        }
        if !self.token_endpoint_auth_methods_supported.iter().all(|am| INDUS_TOKEN_ENDPOINT_AUTH_MATHODS.contains(&am.as_str())) {
                Err(ProviderConfigError::BadTokenEndpointAuthMethodValue)?
        }
        Ok(self)
    }
    pub fn have_token_endpoint_auth_signing_alg_values_supported(&self) ->ConfigResult<&ProviderConfig> {
        if self.token_endpoint_auth_signing_alg_values_supported.is_some() {
            let algref = self.token_endpoint_auth_signing_alg_values_supported.as_ref().unwrap();
            if algref.len() == 0 {
                Err(ProviderConfigError::EmptyTokenEndpointAuthSigningAlgValue)?
            }
            if !algref.iter().all(|sa| INDUS_SIGNING_ALGORITHMS.contains(&sa.as_str())) {
                Err(ProviderConfigError::BadTokenEndpointAuthSigningAlgValue)?
            }
            // servers SHOULD support RS256.
            if !algref.iter().any(|sa| sa.eq("RS256") ) {
                Err(ProviderConfigError::MissingMandatoryTokenEndpointAuthSigningAlgRS256)?
            }
            // "none" MUST NOT be used.
            if algref.iter().any(|sa| sa.eq("none") ) {
                Err(ProviderConfigError::BadTokenEndpointAuthSigningAlgNone)?
            }
        }
        Ok(self)
    }
    pub fn have_consent_display_values(&self) -> ConfigResult<&ProviderConfig> {
        if self.display_values_supported.len() == 0 {
            Err(ProviderConfigError::EmptyConsentDisplayValue)?
        }
        if !self.display_values_supported.iter().all(|dv| INDUS_ENDUSER_CONSENT_DISPLAY_VALUES.contains(&dv.as_str())) {
            Err(ProviderConfigError::BadConsentDisplayValue)?
        }
        if self.display_values_supported.len() == 1 {
            let dv = self.display_values_supported.get(0).unwrap();
            if dv.eq("none") { Err(ProviderConfigError::OnlyNoneConsentDisplayValue)? }
        }
        Ok(self)
    }
    pub fn have_claim_types(&self) -> ConfigResult<&ProviderConfig> {
        let refct = self.claim_types_supported.as_ref().unwrap();
        if !refct.iter().all(|ct| INDUS_CLAIM_TYPES.contains(&ct.as_str())) {
            Err(ProviderConfigError::BadClaimTypeValue)?
        }
        Ok(self)
    }
    pub fn have_claim_names(&self) -> ConfigResult<&ProviderConfig> {
        if self.claims_supported.len() == 0 {
            Err(ProviderConfigError::EmptyClaimNames)?
        }
        if !self.claims_supported.iter().all(|ct| INDUS_CLAIM_NAMES.contains(&ct.as_str())) {
            Err(ProviderConfigError::BadClaimName)?
        }
        Ok(self)
    }
    pub fn have_request_param_uri_registration_values(&self) -> ConfigResult<&ProviderConfig> {
        if !self.request_parameter_supported {
            Err(ProviderConfigError::BadRequestParameter)?
        }
        if !self.request_uri_parameter_supported {
            Err(ProviderConfigError::BadRequestUriParameter)?
        }
        if !self.require_request_uri_registration {
            Err(ProviderConfigError::BadRequireRequestUriRegistration)?
        }
        Ok(self)
    }
}
