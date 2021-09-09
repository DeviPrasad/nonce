#[test]
pub fn config_basic_json_str() {
    let indus_basic = r#"{
        "issuer": "https://www.indus.in/nonce/",
        "authorization_endpoint": "https://www.indus.in/nonce/auth",
        "token_endpoint": "https://www.indus.in/nonce/token",
        "userinfo_endpoint": "https://www.indus.in/nonce/userinfo",
        "jwks_uri": "https://www.indus.in/nonce/oidc/jwks",
        "registration_endpoint": "https://www.indus.in/nonce/oidc/registration",
        "scopes_supported": ["openid", "general", "profile", "email", "address", "phone", "aadhaar", "voterid"],
        "response_types_supported": ["none", "code", "token", "id_token", "token id_token", "id_token token"],
        "response_modes_supported": ["query", "fragment"],
        "grant_types_supported": ["authorization_code", "refresh_token",
            "urn:ietf:params:oauth:grant-type:jwt-bearer", "implicit", "client_credentials", "password"],
        "subject_types_supported": ["public", "pairwise"],
        "id_token_signing_alg_values_supported": ["HS256", "RS256", "ES256", "ES512"],
        "id_token_encryption_alg_values_supported": ["dir", "ECDH-ES+A256KW"],
        "userinfo_signing_alg_values_supported": ["none"],
        "display_values_supported": ["none", "login"],
        "claim_types_supported": [],
        "claims_supported": ["sub", "name", "aadhaar", "voterid", "preferred_username", "picture", "locale", "email", "profile"],
        "request_parameter_supported": true,
        "request_uri_parameter_supported": true,
        "require_request_uri_registration": true,
        "service_documentation": "https://www.indus.in/nonce/oidc/service-doc",
        "op_policy_uri": "https://www.indus.in/nonce/oidc/provider-policy",
        "op_tos_uri": "https://www.indus.in/nonce/oidc/provider-terms-of-service"
        }"#;
    let res = super::config::ProviderConfig::from_json(indus_basic);
    if let Ok(j) = res {
        println!();
        println!("{:#?}", j);
    } else if let Err(e) = res {
        println!();
        println!("Test Failed - error : {:#?}", e);
        println!();
        assert!(false);
    }
}
