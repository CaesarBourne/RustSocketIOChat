use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    username: String,
    exp: i64,
}

pub fn generate_token(username: &str) -> String {
    let claims = Claims {
        username: username.to_string(),
        exp: 10000000000, // expires in 10 seconds
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret_key".as_bytes()),
    )
    .unwrap();
    token
}

pub fn verify_token(token: &str) -> bool {
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret_key".as_bytes()),
        &Validation::default(),
    );
    match token_data {
        Ok(_) => true,
        Err(_) => false,
    }
}
