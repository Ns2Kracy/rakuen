use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

static JWT_SECRET: Lazy<String> =
    Lazy::new(|| dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set"));

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub token: String,
    pub token_type: String,
}

impl AuthBody {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    // Subject
    pub sub: String,
    // Expiration Time
    pub exp: i64,
    // Issuer
    pub iss: String,
    // Not Before
    pub nbf: i64,
}

impl JwtClaims {
    pub fn new(sub: String) -> Self {
        let now = chrono::Utc::now();

        Self {
            sub,
            exp: (now + chrono::Duration::days(30)).timestamp(),
            iss: "Rakuen".to_string(),
            nbf: now.timestamp(),
        }
    }

    pub fn encode_jwt(&self) -> Result<String, AuthError> {
        encode(
            &Header::new(Algorithm::default()),
            &self,
            &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
        )
        .map_err(|_| AuthError::TokenCreation)
    }

    fn decode_jwt(token: &str) -> Result<Self, AuthError> {
        decode::<Self>(
            token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &jsonwebtoken::Validation::new(Algorithm::default()),
        )
        .map(|data| data.claims)
        .map_err(|_| AuthError::InvalidToken)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data =
            JwtClaims::decode_jwt(bearer.token()).map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
