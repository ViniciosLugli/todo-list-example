use std::convert::Infallible;

use crate::{error::HttpError, states::app::AppStateType};

use chrono;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use ntex::{
	http::Payload,
	web::{self, ErrorRenderer, FromRequest, HttpRequest, HttpResponse},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
	uuid: String,
	email: String,
	exp: usize,
}

impl Claims {
	fn new(uuid: String, email: String) -> Self {
		Self { uuid, email, exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize }
	}

	pub fn get_user_uuid(&self) -> String {
		self.uuid.clone()
	}

	pub fn get_user_email(&self) -> String {
		self.email.clone()
	}

	pub fn get_expiration(&self) -> usize {
		self.exp
	}
}

impl<Err: ErrorRenderer> FromRequest<Err> for Claims {
	type Error = Infallible;

	#[inline]
	async fn from_request(req: &HttpRequest, _: &mut Payload) -> Result<Claims, Infallible> {
		Ok(req.extensions().get::<Claims>().unwrap().clone())
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct UserCreateInput {
	name: String,
	email: String,
	password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserLoginInput {
	email: String,
	password: String,
}

#[web::post("/register")]
async fn create_user(
	state: web::types::State<AppStateType>,
	user_input: web::types::Json<UserCreateInput>,
) -> Result<HttpResponse, HttpError> {
	let app_state = state.read().await;

	let user = match app_state
		.repositories
		.user
		.create(user_input.name.clone(), user_input.email.clone(), user_input.password.clone())
		.await
	{
		Ok(user) => user,
		Err(_) => return Err(HttpError::internal_server_error("Failed to create user")),
	};

	Ok(HttpResponse::Created().json(&json!({ "name": user.name, "email": user.email })))
}

#[web::post("/login")]
async fn login_user(
	state: web::types::State<AppStateType>,
	user_input: web::types::Json<UserLoginInput>,
) -> Result<HttpResponse, HttpError> {
	let app_state = state.read().await;

	let user = match app_state.repositories.user.find_by_credentials(user_input.email.clone(), user_input.password.clone()).await
	{
		Ok(Some(user)) => user,
		Ok(None) => return Err(HttpError::unauthorized("Invalid credentials")),
		Err(_) => return Err(HttpError::unauthorized("Invalid credentials")),
	};

	let claims = Claims::new(user.uuid.clone(), user.email.clone());
	let header = Header::new(Algorithm::HS256);
	let key = EncodingKey::from_secret(app_state.secret.as_ref());

	let token = match encode(&header, &claims, &key) {
		Ok(token) => token,
		Err(_) => return Err(HttpError::internal_server_error("Failed to create token")),
	};

	Ok(HttpResponse::Ok().json(&json!({ "token": token , "name": user.name, "email": user.email, "uuid": user.uuid })))
}

#[web::get("/info")]
async fn get_user(state: web::types::State<AppStateType>, claims: Claims) -> Result<HttpResponse, HttpError> {
	let app_state = state.read().await;

	let user = match app_state.repositories.user.find_by_uuid(claims.get_user_uuid()).await {
		Ok(Some(user)) => user,
		Ok(None) => return Err(HttpError::not_found("User not found")),
		Err(_) => return Err(HttpError::internal_server_error("Failed to get user")),
	};

	Ok(HttpResponse::Ok().json(&json!({ "name": user.name, "email": user.email })))
}

pub fn init(config: &mut web::ServiceConfig) {
	config.service(web::scope("/user").service(create_user).service(login_user).service(get_user));
}
