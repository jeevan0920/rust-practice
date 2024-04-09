use actix_identity::Identity;
use actix_web::{dev::ServiceRequest, web, HttpResponse};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (whom the token refers to)
    exp: usize,  // Expiry
}

pub fn create_token(username: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap()
}

pub fn validate_token(token: &str) -> bool {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .is_ok()
}

pub async fn validator(
    req: ServiceRequest,
    credentials: Identity,
) -> Result<ServiceRequest, (ServiceRequest, HttpResponse)> {
    if let Some(identity) = credentials.identity() {
        if validate_token(&identity) {
            Ok(req)
        } else {
            Err((req, HttpResponse::Unauthorized().finish()))
        }
    } else {
        Err((req, HttpResponse::Unauthorized().finish()))
    }
}
