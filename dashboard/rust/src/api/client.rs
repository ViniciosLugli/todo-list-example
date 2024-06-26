use reqwest::Client;
use serde_json::{json, Value};

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub struct APIClient {
    base_url: String,
    client: Client,
    jwt_bearer: Option<String>,
}

impl APIClient {
    pub fn new(base_url: &str) -> Self {
        println!("Creating new APIClient with base_url: {}", base_url);
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
            jwt_bearer: None,
        }
    }

    async fn parse_response(&mut self, response: reqwest::Response) -> Result<String, String> {
        let status = response.status();
        let json: Value = match response.json::<Value>().await {
            Ok(value) => value,
            Err(e) => return Err(e.to_string()),
        };

        let mut json_obj = json.as_object().unwrap().clone();
        json_obj.insert("status_code".to_string(), json!(status.as_u16()));
        let json = json!(json_obj);

        if status.is_client_error() || status.is_server_error() {
            println!("Error: {}", json);
        } else {
            if let Some(token) = json["token"].as_str() {
                self.jwt_bearer = Some(token.to_string());
            }
        }

        Ok(json.to_string())
    }

    pub fn get_base_url(&self) -> String {
        self.base_url.clone()
    }

    pub fn get_jwt_bearer(&self) -> String {
        self.jwt_bearer.clone().unwrap_or_default()
    }

    async fn send_request(
        &self,
        method: reqwest::Method,
        url: String,
        params: Option<serde_json::Value>,
    ) -> Result<reqwest::Response, String> {
        let mut request_builder = self.client.request(method, &url);

        if let Some(token) = &self.jwt_bearer {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
        }

        let request = if let Some(params) = params {
            request_builder.json(&params).build()
        } else {
            request_builder.build()
        }
        .map_err(|e| e.to_string())?;

        self.client
            .execute(request)
            .await
            .map_err(|e| e.to_string())
    }

    async fn send_get_request(&self, url: String) -> Result<reqwest::Response, String> {
        self.send_request(reqwest::Method::GET, url, None).await
    }

    async fn send_post_request(
        &self,
        url: String,
        params: serde_json::Value,
    ) -> Result<reqwest::Response, String> {
        self.send_request(reqwest::Method::POST, url, Some(params))
            .await
    }

    async fn send_put_request(
        &self,
        url: String,
        params: serde_json::Value,
    ) -> Result<reqwest::Response, String> {
        self.send_request(reqwest::Method::PUT, url, Some(params))
            .await
    }

    async fn send_delete_request(&self, url: String) -> Result<reqwest::Response, String> {
        self.send_request(reqwest::Method::DELETE, url, None).await
    }

    pub async fn register_user(
        &mut self,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<String, String> {
        let url = format!("{}/user/register", self.base_url);
        let params = json!({
            "name": name,
            "email": email,
            "password": password,
        });
        let response = self.send_post_request(url, params).await?;
        self.parse_response(response).await
    }

    pub async fn login_user(&mut self, email: &str, password: &str) -> Result<String, String> {
        let url = format!("{}/user/login", self.base_url);
        let params = json!({
            "email": email,
            "password": password,
        });
        let response = self.send_post_request(url, params).await?;

        self.parse_response(response).await
    }

    pub async fn get_user_info(&mut self) -> Result<String, String> {
        let url = format!("{}/user/info", self.base_url);
        let response = self.send_get_request(url).await?;
        self.parse_response(response).await
    }

    pub async fn create_task(&mut self, title: &str, description: &str) -> Result<String, String> {
        let url = format!("{}/task/", self.base_url);
        let params = json!({
            "title": title,
            "description": description,
        });
        let response = self.send_post_request(url, params).await?;
        self.parse_response(response).await
    }

    pub async fn update_task(
        &mut self,
        cuid: &str,
        title: &str,
        description: &str,
        done: bool,
    ) -> Result<String, String> {
        let url = format!("{}/task/{}", self.base_url, cuid);
        let params = json!({
            "title": title,
            "description": description,
            "done": done,
        });
        let response = self.send_put_request(url, params).await?;
        self.parse_response(response).await
    }

    pub async fn delete_task(&mut self, cuid: &str) -> Result<String, String> {
        let url = format!("{}/task/{}", self.base_url, cuid);
        let response = self.send_delete_request(url).await?;
        self.parse_response(response).await
    }

    pub async fn find_task(&mut self, cuid: &str) -> Result<String, String> {
        let url = format!("{}/task/{}", self.base_url, cuid);
        let response = self.send_get_request(url).await?;
        self.parse_response(response).await
    }

    pub async fn find_all_tasks(&mut self) -> Result<String, String> {
        let url = format!("{}/task/", self.base_url);
        let response = self.send_get_request(url).await?;
        self.parse_response(response).await
    }
}
