use ntex::{
	http,
	web::{self, HttpResponse},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpError {
	pub message: Value,

	#[serde(skip)]
	pub status: http::StatusCode,
}

impl HttpError {
	pub fn new(status: http::StatusCode, message: &str) -> Self {
		Self { status, message: json!({ "error": message }) }
	}

	pub fn bad_request(message: &str) -> Self {
		Self::new(http::StatusCode::BAD_REQUEST, message)
	}

	pub fn unauthorized(message: &str) -> Self {
		Self::new(http::StatusCode::UNAUTHORIZED, message)
	}

	pub fn forbidden(message: &str) -> Self {
		Self::new(http::StatusCode::FORBIDDEN, message)
	}

	pub fn not_found(message: &str) -> Self {
		Self::new(http::StatusCode::NOT_FOUND, message)
	}

	pub fn internal_server_error(message: &str) -> Self {
		Self::new(http::StatusCode::INTERNAL_SERVER_ERROR, message)
	}

	pub fn service_unavailable(message: &str) -> Self {
		Self::new(http::StatusCode::SERVICE_UNAVAILABLE, message)
	}
}

impl std::fmt::Display for HttpError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}] {}", self.status, self.message)
	}
}

impl std::error::Error for HttpError {}

impl web::WebResponseError for HttpError {
	fn error_response(&self, _: &web::HttpRequest) -> HttpResponse {
		HttpResponse::build(self.status).json(&self.message)
	}
}
