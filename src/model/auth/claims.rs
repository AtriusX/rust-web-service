use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub exp: usize,
}

impl Display for JwtClaims {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sub: {}\nExpire: {}", self.sub, self.exp)
    }
}

// impl<S> FromRequestParts<S> for JwtClaims
// where
//     S: Send + Sync,
// {
//     type Rejection = AuthError;
//
//     async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
//         let TypedHeader(Authorization(bearer)) = parts
//             .extract::<TypedHeader<Authorization<Bearer>>>()
//             .await
//             .map_err(|_| AuthError::InvalidToken)?;
//         let token_data = decode::<JwtClaims>(
//             bearer.token(),
//             &KEYS.decoding,
//             &Validation::default(),
//         )
//             .map_err(|_| AuthError::InvalidToken)?;
//         Ok(token_data.claims)
//     }
// }