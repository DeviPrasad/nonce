pub mod digest {
    use ring::digest::{Context, Digest, SHA256};
    use data_encoding::HEXUPPER;

    pub fn sha256_u8arr(input: &[u8]) -> Digest {
        let mut context = Context::new(&SHA256);
        context.update(input);
        context.finish()
    }

    pub fn sha256_str(input: &str) -> Digest {
        let mut context = Context::new(&SHA256);
        context.update(input.as_bytes());
        context.finish()
    }

    pub fn hex_upcase(digest: Digest) -> String {
        HEXUPPER.encode(digest.as_ref())
    }

}

pub mod base64 {
        use std::str;
        use data_encoding::{BASE64URL_NOPAD, DecodeError};

    pub fn base64_encode_str(s: &str) -> String {
        BASE64URL_NOPAD.encode(s.as_bytes())
    }

    pub fn base64_encode_u8arr(input: &[u8]) -> String {
        BASE64URL_NOPAD.encode(input)
    }

    pub fn base64_decode_str(s: &str) -> Result<String, bool> {
        let res: Result<Vec<u8>, DecodeError> = BASE64URL_NOPAD.decode(s.as_bytes());
        if let Ok(dec) = res {
            let decstr = str::from_utf8(&dec).unwrap();
            return Ok(decstr.to_string());
        } else {
            Err(false)
        }
    }
}
