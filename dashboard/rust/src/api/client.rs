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

    async fn parse_response(&self, response: reqwest::Response) -> Result<String, String> {
        let status = response.status();
        let json: Value;

        match response.json::<Value>().await {
            Ok(value) => json = value,
            Err(e) => return Err(e.to_string()),
        }

        let mut json_obj = json.as_object().unwrap().clone();
        json_obj.insert("status_code".to_string(), json!(status.as_u16()));
        let json = json!(json_obj);

        if status.is_client_error() || status.is_server_error() {
            println!("Error: {}", json);
        }
        Ok(json.to_string())
    }

    pub fn get_base_url(&self) -> String {
        self.base_url.clone()
    }

    pub fn get_jwt_bearer(&self) -> String {
        self.jwt_bearer.clone().unwrap_or_default()
    }

    pub async fn register_user(
        &self,
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
        let response = self
            .client
            .post(&url)
            .json(&params)
            .send()
            .await
            .map_err(|e| e.to_string())
            .map_err(|e| e.to_string())?;
        self.parse_response(response).await
    }

    pub async fn login_user(&mut self, email: &str, password: &str) -> Result<String, String> {
        let url = format!("{}/user/login", self.base_url);
        let params = json!({
            "email": email,
            "password": password,
        });
        let response = self
            .client
            .post(&url)
            .json(&params)
            .send()
            .await
            .map_err(|e| e.to_string())
            .map_err(|e| e.to_string())?;

        let json: Value;
        let status = response.status();
        match response.json::<Value>().await {
            Ok(value) => json = value,
            Err(e) => return Err(e.to_string()),
        }

        let mut json_obj = json.as_object().unwrap().clone();
        json_obj.insert("status_code".to_string(), json!(status.as_u16()));
        let json = json!(json_obj);

        if status.is_success() {
            let token = json["token"].as_str().unwrap();
            self.jwt_bearer = Some(token.to_string());
        }

        Ok(json.to_string())
    }

    pub async fn get_user_info(&self) -> Result<String, String> {
        let url = format!("{}/user/info", self.base_url);
        let response = self
            .client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.jwt_bearer.as_ref().unwrap()),
            )
            .send()
            .await
            .map_err(|e| e.to_string())?;
        self.parse_response(response).await
    }

    pub async fn create_task(&self, title: &str, description: &str) -> Result<String, String> {
        let url = format!("{}/task/", self.base_url);
        let params = json!({
            "title": title,
            "description": description,
        });
        let response = self
            .client
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.jwt_bearer.as_ref().unwrap()),
            )
            .json(&params)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        self.parse_response(response).await
    }

    pub async fn update_task(
        &self,
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
        let response = self
            .client
            .put(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.jwt_bearer.as_ref().unwrap()),
            )
            .json(&params)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        self.parse_response(response).await
    }

    pub async fn delete_task(&self, cuid: &str) -> Result<String, String> {
        let url = format!("{}/task/{}", self.base_url, cuid);
        let response = self
            .client
            .delete(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.jwt_bearer.as_ref().unwrap()),
            )
            .send()
            .await
            .map_err(|e| e.to_string())?;
        self.parse_response(response).await
    }

    pub async fn find_task(&self, cuid: &str) -> Result<String, String> {
        let url = format!("{}/task/{}", self.base_url, cuid);
        let response = self
            .client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.jwt_bearer.as_ref().unwrap()),
            )
            .send()
            .await
            .map_err(|e| e.to_string())?;
        self.parse_response(response).await
    }

    pub async fn find_all_tasks(&self) -> Result<String, String> {
        let url = format!("{}/task/", self.base_url);
        let response = self
            .client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.jwt_bearer.as_ref().unwrap()),
            )
            .send()
            .await
            .map_err(|e| e.to_string())?;
        self.parse_response(response).await
    }
}
