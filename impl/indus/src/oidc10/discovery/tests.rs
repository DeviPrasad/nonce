#[test]
pub fn config_basic_json_str() {
    let indus_basic = r#"{
        "issuer": "https://www.indus.in/nonce/",
        "authorization_endpoint": "https://www.indus.in/nonce/auth",
        "token_endpoint": "https://www.indus.in/nonce/token",
        "userinfo_endpoint": "https://www.indus.in/nonce/userinfo",
        "jwks_uri": "https://www.indus.in/nonce/oidc/jwks",
        "registration_endpoint": "https://www.indus.in/nonce/oidc/registration",
        "scopes_supported": ["openid", "general", "profile", "email", "address", "phone"],
        "response_types_supported": ["code", "id_token", "token id_token", "id_token token"],
        "response_modes_supported": ["query", "fragment"],
        "grant_types_supported": ["authorization_code", "implicit", "refresh_token",
            "urn:ietf:params:oauth:grant-type:jwtbearer", "client_credentials", "password"],
        "acr_values_supported": [],
        "subject_types_supported": ["public", "public"],
        "id_token_signing_alg_values_supported": ["none", "RS256", "HS256"],
        "claim_types_supported": ["normal", "abcd"],
        "claims_supported": ["sub", "name", "aadhaar", "voterid", "groupIds", "preferred_username", "picture", "locale", "email", "profile"],
        "request_parameter_supported": true,
        "request_uri_parameter_supported": true,
        "require_request_uri_registration": true,
        "service_documentation": "https://www.indus.in/nonce/oidc/service-doc",
        "op_policy_uri": "https://www.indus.in/nonce/oidc/provider-policy",
        "op_tos_uri": "https://www.indus.in/nonce/oidc/provider-terms-of-service"
        }"#;
    let res = super::config::ProviderConfig::from_json(indus_basic);
    assert!(res.is_ok());
    if let Ok(j) = res {
        println!();
        log::info!("{:#?}", j);
    }
}
