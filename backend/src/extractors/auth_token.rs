use std::future::Ready;

use actix_web::{dev::Payload, Error as ActixWebError, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthToken {
    pub id: usize,
}

impl FromRequest for AuthToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {}
}
