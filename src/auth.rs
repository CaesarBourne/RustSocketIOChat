// auth.rs
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::collections::HashMap;

pub struct Auth {
    secret_key: String,
}

impl Auth {
    pub fn new(secret_key: String) -> Self {
        Auth { secret_key }
    }

    pub fn generate_token(&self, user_id: &str) -> String {
        let mut payload = HashMap::new();
        payload.insert("user_id", user_id);
        let token = encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(self.secret_key.as_bytes()),
        )
        .unwrap();
        token
    }

    pub fn verify_token(&self, token: &str) -> bool {
        let validation = Validation::default();
        let token_data = decode::<HashMap<String, String>>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &validation,
        );
        match token_data {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
