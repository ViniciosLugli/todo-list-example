pub struct ResponseBuilder {
	status_code: &'static str,
	content_type: &'static str,
	body: String,
}

impl ResponseBuilder {
	pub fn new(status_code: &'static str, content_type: &'static str, body: String) -> Self {
		ResponseBuilder { status_code, content_type, body }
	}

	pub fn json(body: serde_json::Value) -> Self {
		ResponseBuilder::new("200 OK", "application/json", body.to_string())
	}

	pub fn not_found() -> Self {
		ResponseBuilder::new("404 NOT FOUND", "application/json", "{\"error\": \"Not Found\"}".to_string())
	}

	pub fn unauthorized() -> Self {
		ResponseBuilder::new("401 UNAUTHORIZED", "application/json", "{\"error\": \"Unauthorized\"}".to_string())
	}

	pub fn bad_request(json: serde_json::Value) -> Self {
		ResponseBuilder::new("400 BAD REQUEST", "application/json", json.to_string())
	}

	pub fn conflict(json: serde_json::Value) -> Self {
		ResponseBuilder::new("409 CONFLICT", "application/json", json.to_string())
	}

	pub fn build(&self) -> String {
		format!("HTTP/1.1 {}\r\nContent-Type: {}\r\n\r\n{}", self.status_code, self.content_type, self.body)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;

	#[test]
	fn test_response_builder_json() {
		let response = ResponseBuilder::json(json!({"status": "ok"})).build();
		assert_eq!(response, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"status\":\"ok\"}");
	}

	#[test]
	fn test_response_builder_not_found() {
		let response = ResponseBuilder::not_found().build();
		assert_eq!(response, "HTTP/1.1 404 NOT FOUND\r\nContent-Type: application/json\r\n\r\n{\"error\": \"Not Found\"}");
	}

	#[test]
	fn test_response_builder_unauthorized() {
		let response = ResponseBuilder::unauthorized().build();
		assert_eq!(response, "HTTP/1.1 401 UNAUTHORIZED\r\nContent-Type: application/json\r\n\r\n{\"error\": \"Unauthorized\"}");
	}

	#[test]
	fn test_response_builder_bad_request() {
		let response = ResponseBuilder::bad_request(json!({"error": "Bad Request"})).build();
		assert_eq!(response, "HTTP/1.1 400 BAD REQUEST\r\nContent-Type: application/json\r\n\r\n{\"error\":\"Bad Request\"}");
	}

	#[test]
	fn test_response_builder_conflict() {
		let response = ResponseBuilder::conflict(json!({"error": "Conflict"})).build();
		assert_eq!(response, "HTTP/1.1 409 CONFLICT\r\nContent-Type: application/json\r\n\r\n{\"error\":\"Conflict\"}");
	}

	#[test]
	fn test_response_builder_build() {
		let response = ResponseBuilder::new("200 OK", "text/plain", "Hello, World!".to_string()).build();
		assert_eq!(response, "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!");
	}

	#[test]
	fn test_response_builder_build_empty() {
		let response = ResponseBuilder::new("200 OK", "text/plain", "".to_string()).build();
		assert_eq!(response, "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n");
	}
}
