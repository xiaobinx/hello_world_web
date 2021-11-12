use actix_web::{error, Error, HttpRequest};
use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{de::DeserializeOwned, Serialize};
use sha2::Sha256;

pub struct TokenTool {
    hmac: Hmac<Sha256>,
}

impl TokenTool {
    pub fn new(key: &[u8]) -> TokenTool {
        let hmac: Hmac<Sha256> = Hmac::new_from_slice(key).unwrap();
        TokenTool { hmac }
    }

    pub fn sign<T: Serialize>(&self, t: &T) -> Result<String, Error> {
        let token = t
            .sign_with_key(&self.hmac)
            .map_err(|e| error::ErrorInternalServerError(e))?;
        Ok(token)
    }

    pub fn verify_from_req<T: DeserializeOwned>(&self, req: &HttpRequest) -> Result<T, Error> {
        let auth = req.headers().get("Authorization");
        match auth {
            None => Err(error::ErrorUnauthorized("Unauthorized")),
            Some(auth) => {
                let token = auth.to_str().unwrap();
                Ok(self.verify_from_str(token)?)
            }
        }
    }

    pub fn verify_from_str<T: DeserializeOwned>(&self, token: &str) -> Result<T, Error> {
        let t: T = token
            .verify_with_key(&self.hmac)
            .map_err(|e| error::ErrorInternalServerError(e))?;
        Ok(t)
    }
}
