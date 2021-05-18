pub struct TokenTool {
    hmac: Hmac<Sha256>,
}

impl TokenTool {
    pub fn new(key: &[u8]) -> TokenTool {
        let hmac: Hmac<Sha256> = Hmac::new_varkey(key).unwrap();
        TokenTool { hmac }
    }

    pub fn sign(&self, t: &TokenInfo) -> Result<String, Error> {
        let token = t
            .sign_with_key(&self.hmac)
            .map_err(|e| error::ErrorInternalServerError(e))?;
        Ok(token)
    }

    pub fn verify_from_req(&self, req: &HttpRequest) -> Result<TokenInfo, Error> {
        let auth = req.headers().get("Authorization");
        match auth {
            None => Err(error::ErrorUnauthorized("Unauthorized")),
            Some(auth) => {
                let token = auth.to_str().unwrap();
                Ok(self.verify_from_str(token)?)
            }
        }
    }

    pub fn verify_from_str(&self, token: &str) -> Result<TokenInfo, Error> {
        let t: TokenInfo = token
            .verify_with_key(&self.hmac)
            .map_err(|e| error::ErrorInternalServerError(e))?;
        Ok(t)
    }
}
