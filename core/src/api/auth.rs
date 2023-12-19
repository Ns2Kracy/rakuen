use crate::{
    common::{
        auth::{AuthBody, JwtClaims},
        crypto::{get_random_cost, hash_password, verify_password},
    },
    context::Ctx,
    errors::{ApiError, ApiResult},
};
use axum::{routing::post, Extension, Json, Router};
use serde::Deserialize;
use validator::Validate;

pub fn mount() -> Router {
    Router::new()
    // .nest(
        // "/auth",
        // Router::new()
        //     .route("/register", post(regiseter))
        //     .route("/login", post(login)),
    // )
}

// #[derive(Validate, Deserialize)]
// struct LoginAndRegisterPaylod {
//     #[validate(length(min = 3, max = 20))]
//     pub username: String,
//     #[validate(length(min = 8, max = 32))]
//     pub password: String,
// }

// async fn regiseter(
//     Extension(ctx): Extension<Ctx>,
//     Json(payload): Json<LoginAndRegisterPaylod>,
// ) -> ApiResult<Json<user::Model>> {
//     let validation = payload.validate();

//     if let Err(e) = validation {
//         return Err(ApiError::ValidationError(e));
//     }
//     todo!()
// }

// async fn login(
//     Extension(ctx): Extension<Ctx>,
//     Json(payload): Json<LoginAndRegisterPaylod>,
// ) -> ApiResult<Json<AuthBody>> {
//     let validation = payload.validate();

//     if let Err(e) = validation {
//         return Err(ApiError::ValidationError(e));
//     }

//     todo!()
// }
