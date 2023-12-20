use crate::{
    common::{
        auth::{AuthBody, JwtClaims},
        crypto::{generate_salt, hash_password, verify_password},
        uuid::new_uuid,
    },
    context::Ctx,
    errors::{ApiError, ApiResult},
};
use axum::{routing::post, Extension, Json, Router};
use rakuen_enity::{user, User};
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter, TryIntoModel};
use serde::Deserialize;
use validator::Validate;

pub fn mount() -> Router {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/register", post(regiseter))
            .route("/login", post(login)),
    )
}

#[derive(Validate, Deserialize)]
struct LoginAndRegisterPayload {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
}

async fn regiseter(
    Extension(ctx): Extension<Ctx>,
    Json(payload): Json<LoginAndRegisterPayload>,
) -> ApiResult<Json<user::Model>> {
    let validation = payload.validate();

    if let Err(e) = validation {
        return Err(ApiError::ValidationError(e));
    }

    let check_registered: Option<user::Model> = User::find()
        .filter(user::Column::Username.contains(payload.username.clone()))
        .one(&ctx.db)
        .await?;

    if check_registered.is_some() {
        return Err(ApiError::BadRequest("User already exists".to_string()));
    }

    let salt = generate_salt();
    let hashed_password = hash_password(&payload.password, &salt)
        .map_err(|e| ApiError::InternalServerError(format!("Failed to hash password: {}", e)))?;

    let date_time = chrono::Local::now();
    let native_utc = date_time.naive_utc();
    let offset = *date_time.offset();
    let time = chrono::DateTime::from_naive_utc_and_offset(native_utc, offset);

    let new_user = user::ActiveModel {
        id: ActiveValue::set(new_uuid()),
        username: ActiveValue::Set(payload.username.clone()),
        password: ActiveValue::Set(hashed_password),
        salt: ActiveValue::Set(salt),
        created_at: ActiveValue::Set(time),
        updated_at: ActiveValue::Set(time),
    };

    if let Err(e) = User::insert(new_user.clone()).exec(&ctx.db).await {
        return Err(ApiError::InternalServerError(format!(
            "Failed to insert user: {}",
            e
        )));
    }

    let result = user::ActiveModel {
        password: ActiveValue::Set("".to_string()),
        salt: ActiveValue::Set("".to_string()),
        ..new_user
    }
    .try_into_model()
    .unwrap();

    Ok(Json(result))
}

async fn login(
    Extension(ctx): Extension<Ctx>,
    Json(payload): Json<LoginAndRegisterPayload>,
) -> ApiResult<Json<AuthBody>> {
    let validation = payload.validate();

    if let Err(e) = validation {
        return Err(ApiError::ValidationError(e));
    }

    // check if username is already taken
    let check_exists: Option<user::Model> = User::find()
        .filter(user::Column::Username.contains(payload.username.clone()))
        .one(&ctx.db)
        .await?;

    if check_exists.is_none() {
        return Err(ApiError::BadRequest("User does not exist".to_string()));
    }

    let user = User::find()
        .filter(user::Column::Username.contains(payload.username.clone()))
        .one(&ctx.db)
        .await?
        .unwrap();

    let verify = verify_password(&payload.password, &user.password, &user.salt)
        .map_err(|e| ApiError::InternalServerError(format!("Failed to verify password: {}", e)))?;

    if !verify {
        return Err(ApiError::BadRequest("Invalid password".to_string()));
    }

    let token = JwtClaims::new(user.id)
        .encode_jwt()
        .map_err(|e| ApiError::InternalServerError(format!("Failed to encode JWT: {}", e)))?;

    Ok(Json(AuthBody::new(token)))
}
