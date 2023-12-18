use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/auth", Router::new())
}

// #[derive(Validate, Deserialize)]
// struct LoginAndRegisterPaylod {
//     #[validate(length(min = 3, max = 20))]
//     pub username: String,
//     #[validate(length(min = 8, max = 32))]
//     pub password: String,
// }

// async fn regiseter(
//     State(ctx): State<Ctx>,
//     Json(payload): Json<LoginAndRegisterPaylod>,
// ) -> ApiResult<Json<User>> {
//     let validation = payload.validate();

//     if let Err(e) = validation {
//         return Err(ApiError::ValidationError(e));
//     }

//     // check if user already exists
//     let check_registered = ctx
//         .db
//         .user()
//         .find_first(vec![user::username::equals(payload.username.clone())])
//         .exec()
//         .await?;

//     if check_registered.is_some() {
//         return Err(ApiError::BadRequest("User already exists".to_string()));
//     }

//     // hash password
//     let cost = get_random_cost();
//     let hashed_password = hash_password(&payload.password, cost).unwrap();

//     // create user
//     let user = ctx
//         .db
//         .user()
//         .create(payload.username, hashed_password, vec![])
//         .exec()
//         .await?;

//     // reset password to avoid sending it back to the client
//     let user = User {
//         hashed_password: "".to_string(),
//         ..user.into()
//     };

//     Ok(Json(user))
// }

// async fn login(
//     State(ctx): State<Ctx>,
//     Json(payload): Json<LoginAndRegisterPaylod>,
// ) -> ApiResult<Json<AuthBody>> {
//     let validation = payload.validate();

//     if let Err(e) = validation {
//         return Err(ApiError::ValidationError(e));
//     }

//     // check if user exists
//     let check_exists = ctx
//         .db
//         .user()
//         .find_first(vec![user::username::equals(payload.username.clone())])
//         .exec()
//         .await?;

//     if check_exists.is_none() {
//         return Err(ApiError::BadRequest("User does not exist".to_string()));
//     }

//     // fetch user
//     let user = ctx
//         .db
//         .user()
//         .find_unique(user::UniqueWhereParam::UsernameEquals(
//             payload.username.clone(),
//         ))
//         .exec()
//         .await?
//         .unwrap();

//     let verify = verify_password(&payload.password, &user.hashed_password).unwrap();

//     if !verify {
//         return Err(ApiError::BadRequest("Invalid password".to_string()));
//     }

//     // create token
//     let token = JwtClaims::new(user.id)
//         .encode_jwt()
//         .map_err(|e| ApiError::InternalServerError(format!("Failed to encode JWT: {}", e)))?;

//     Ok(Json(AuthBody::new(token)))
// }
