use std::future::{ready, Ready};

use actix_web::{
    dev::Payload,
    error::ErrorUnauthorized,
    http::{self, header::HeaderValue},
    web::Data,
    Error as ActixWebError, FromRequest, HttpRequest,
};
use jsonwebtoken::{
    decode, errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

use crate::scopes::user::Claims;

#[derive(Serialize, Deserialize)]
pub struct AuthToken {
    pub id: usize,
}

impl FromRequest for AuthToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let auth_header: Option<&HeaderValue> = req.headers().get(http::header::AUTHORIZATION);
        let auth_token: String = auth_header.unwrap().to_str().unwrap_or("").to_string();

        if auth_token.is_empty() {
            return Err(ErrorUnauthorized("Invaild auth token!"));
        }

        let secret: String = req.app_data::<Data<String>>().unwrap().to_string();

        let decode: Result<TokenData<Claims>, JwtError> = decode(
            &auth_token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => ready(Ok(AuthToken {
                id: token.claims.id,
            })),
            Err(e) => ready(Err(ErrorUnauthorized(e))),
        }
    }
}
