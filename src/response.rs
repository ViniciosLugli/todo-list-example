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
