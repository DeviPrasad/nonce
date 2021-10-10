#[cfg(test)]
pub mod tests {
    use crate::core::digest;
    use crate::core::base64;
    use ring::digest::Digest;

    #[test]
    fn test_hash_hello_dot_world() {
        let hash: Digest = digest::sha256_str("hello.world");
        let hashstr = digest::hex_upcase(hash);
        assert_eq!(hashstr, ("cb0d83559f6f26c6862373dc5eaed6a3da59cfff864e4d700f0656734aa91018".to_ascii_uppercase()));
    }

    #[test]
    fn test_base64_urlencode_hello_world() {
        assert_eq!("SGVsbG8sIHdvcmxkIQ", base64::base64_encode_str("Hello, world!"));
    }

    #[test]
    fn test_base64_urlencode_indus_url() {
        assert_eq!(base64::base64_encode_str("https://indus.oauth.in/authorize?response_type=code"), "aHR0cHM6Ly9pbmR1cy5vYXV0aC5pbi9hdXRob3JpemU_cmVzcG9uc2VfdHlwZT1jb2Rl");
    }

    #[test]
    fn test_base64_urlencode_decode_indus_url() {
        let url = "https://indus.oauth.in/authorize?response_type=code";
        let enc_expect = "aHR0cHM6Ly9pbmR1cy5vYXV0aC5pbi9hdXRob3JpemU_cmVzcG9uc2VfdHlwZT1jb2Rl";
        let enc: String = base64::base64_encode_str(url);
        assert_eq!(enc_expect, enc);
        assert_eq!(base64::base64_decode_str(&enc).unwrap(), url);
    }
}
