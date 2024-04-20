use base64::{engine::general_purpose, Engine as _};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::response::ResponseBuilder;
use crate::task::Task;
use crate::user::User;

#[derive(Clone)]
pub struct Server {
	tasks: Arc<Mutex<HashMap<usize, Task>>>,
	next_id: Arc<Mutex<usize>>,
	users: Arc<Mutex<HashMap<String, User>>>,
}

impl Server {
	pub fn new() -> Server {
		Server { tasks: Arc::new(Mutex::new(HashMap::new())), next_id: Arc::new(Mutex::new(1)), users: Arc::new(Mutex::new(HashMap::new())) }
	}

	pub fn handle_request(&self, request: &str) -> String {
		let body_start_index = request.find("\r\n\r\n").map(|i| i + 4).unwrap_or(request.len());
		let body = request[body_start_index..].trim_matches(char::from(0)).trim();

		let request_line_end_index = request.find('\n').unwrap_or(0);
		let (method, path) =
			request[..request_line_end_index].split_once(' ').map(|(m, rest)| (m, rest.split_once(' ').map_or("", |(p, _)| p))).unwrap_or_default();

		let headers: HashMap<_, _> =
			request[..body_start_index].lines().skip(1).filter_map(|line| line.split_once(':').map(|(k, v)| (k.trim(), v.trim()))).collect();

		let body_json: serde_json::Value = serde_json::from_str(body).unwrap_or_default();

		match (method, path) {
			("POST", "/users") => {
				if body.is_empty() {
					return ResponseBuilder::bad_request(json!({"error": "Empty body"})).build();
				}

				let username = match body_json.get("username").and_then(|username| username.as_str()) {
					Some(username) => username,
					None => return ResponseBuilder::bad_request(json!({"error": "Missing username"})).build(),
				};

				let password = match body_json.get("password").and_then(|password| password.as_str()) {
					Some(password) => password,
					None => return ResponseBuilder::bad_request(json!({"error": "Missing password"})).build(),
				};

				if username.is_empty() || password.is_empty() {
					return ResponseBuilder::json(json!({"error": "Missing username or password"})).build();
				}

				let mut users = self.users.lock().unwrap();
				if users.contains_key(username) {
					return ResponseBuilder::conflict(json!({"error": "User already exists"})).build();
				}

				let user = User::new(&username, &password);
				users.insert(user.username.clone(), user);

				ResponseBuilder::json(json!({"status": "User created"})).build()
			}

			_ if headers.contains_key("Authorization") => {
				let auth_header = headers.get("Authorization").unwrap();
				debug!("Authorization header: {}", auth_header);
				if !auth_header.starts_with("Basic ") {
					return ResponseBuilder::unauthorized().build();
				}

				let credentials = general_purpose::STANDARD.decode(auth_header[6..].as_bytes()).unwrap_or_default();
				let credentials = String::from_utf8(credentials).unwrap_or_default();
				let (username, password) = credentials.split_once(':').unwrap_or_default();
				let users = self.users.lock().unwrap();
				if let Some(user) = users.get(username).filter(|user| user.authenticate(password)) {
					info!("Authenticated user: {}", user.username);
					match (method, path) {
						("POST", "/tasks") => {
							let mut tasks = self.tasks.lock().unwrap();
							let mut next_id = self.next_id.lock().unwrap();
							let content = body_json.get("content").cloned().unwrap_or_default().to_string();

							if content.is_empty() {
								return ResponseBuilder::bad_request(json!({"error": "Empty body"})).build();
							}

							let task = Task::new(*next_id, content, &user);
							tasks.insert(*next_id, task);
							*next_id += 1;

							ResponseBuilder::json(json!({"status": "Task created"})).build()
						}
						("GET", "/tasks") => {
							let tasks = self.tasks.lock().unwrap();
							let list: Vec<_> = tasks.values().cloned().collect();
							ResponseBuilder::json(serde_json::to_value(list).unwrap()).build()
						}
						("PUT", path) if path.starts_with("/tasks/") => {
							let id = path[7..].parse::<usize>().unwrap();
							if body.is_empty() {
								return ResponseBuilder::bad_request(json!({"error": "Empty body"})).build();
							}

							let mut tasks = self.tasks.lock().unwrap();
							if let Some(task) = tasks.get_mut(&id) {
								info!("Updating task: {}, with user: {}", task.id, task.owner.username);
								if task.owner.username == user.username {
									if let Some(content) = body_json.get("content").and_then(|content| content.as_str()) {
										task.content = content.to_string();
									}

									if let Some(completed) = body_json.get("completed").and_then(|completed| completed.as_bool()) {
										task.completed = completed;
									}

									ResponseBuilder::json(json!({"status": "Task updated"})).build()
								} else {
									info!("User: {} is not the owner of task: {}, owned by: {}", user.username, task.id, task.owner.username);
									ResponseBuilder::unauthorized().build()
								}
							} else {
								ResponseBuilder::not_found().build()
							}
						}
						("DELETE", path) if path.starts_with("/tasks/") => {
							let id = path[7..].parse::<usize>().unwrap();
							let mut tasks = self.tasks.lock().unwrap();
							if let Some(task) = tasks.get(&id) {
								if task.owner.username == user.username {
									tasks.remove(&id);
									ResponseBuilder::json(json!({"status": "Task deleted"})).build()
								} else {
									ResponseBuilder::unauthorized().build()
								}
							} else {
								ResponseBuilder::not_found().build()
							}
						}
						_ => ResponseBuilder::not_found().build(),
					}
				} else {
					ResponseBuilder::not_found().build()
				}
			}
			_ => ResponseBuilder::unauthorized().build(),
		}
	}
}

#[cfg(test)]
mod server_tests {
	use super::*;

	fn setup_user_and_task(server: &Server) -> (String, usize) {
		let response = server.handle_request("POST /users HTTP/1.1\r\n\r\n{\"username\":\"testuser\",\"password\":\"testpass\"}");
		assert!(response.contains("User created"));

		let base64_credentials = general_purpose::STANDARD.encode("testuser:testpass".as_bytes());
		let response =
			server.handle_request("POST /tasks HTTP/1.1\r\nAuthorization: Basic dGVzdHVzZXI6dGVzdHBhc3M=\r\n\r\n{\"content\":\"test task\"}");
		assert!(response.contains("Task created"));

		let tasks = server.tasks.lock().unwrap();
		let task_id = tasks.keys().next().cloned().unwrap();
		(String::from("Authorization: Basic ") + &base64_credentials, task_id)
	}

	#[test]
	fn test_handle_request_post_users() {
		let server = Server::new();
		let response = server.handle_request("POST /users HTTP/1.1\r\n\r\n{\"username\":\"testuser\",\"password\":\"testpass\"}");
		assert!(response.contains("User created"));
	}

	#[test]
	fn test_handle_request_get_tasks_unauthorized() {
		let server = Server::new();
		let response = server.handle_request("GET /tasks HTTP/1.1\r\n\r\n");
		assert!(response.contains("401 UNAUTHORIZED"));
	}

	#[test]
	fn test_authentication_flow() {
		let server = Server::new();
		let (auth_header, _task_id) = setup_user_and_task(&server);
		let get_response = server.handle_request(&format!("GET /tasks HTTP/1.1\r\n{}\r\n\r\n", auth_header));
		assert!(get_response.contains("200 OK"));
	}

	#[test]
	fn test_handle_request_post_tasks() {
		let server = Server::new();
		let (auth_header, _task_id) = setup_user_and_task(&server);
		let post_response = server.handle_request(&format!("POST /tasks HTTP/1.1\r\n{}\r\n\r\n{{\"content\":\"another task\"}}", auth_header));
		assert!(post_response.contains("Task created"));
	}

	#[test]
	fn test_handle_request_put_tasks() {
		let server = Server::new();
		let (auth_header, task_id) = setup_user_and_task(&server);

		let put_response = server.handle_request(&format!(
			"PUT /tasks/{} HTTP/1.1\r\n{}\r\n\r\n{{\"content\":\"updated task\", \"completed\": true}}",
			task_id, auth_header
		));
		assert!(put_response.contains("Task updated"), "Response was: {}", put_response);
	}

	#[test]
	fn test_handle_request_delete_tasks() {
		let server = Server::new();
		let (auth_header, task_id) = setup_user_and_task(&server);

		let delete_response = server.handle_request(&format!("DELETE /tasks/{} HTTP/1.1\r\n{}\r\n\r\n", task_id, auth_header));
		assert!(delete_response.contains("Task deleted"), "Response was: {}", delete_response);
	}
}
