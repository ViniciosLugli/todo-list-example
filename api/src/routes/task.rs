use crate::{error::HttpError, routes::user::Claims, states::app::AppStateType};
use ntex::web::{self, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCreateInput {
	pub title: String,
	pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskUpdateInput {
	pub title: String,
	pub description: String,
	pub done: bool,
}

#[web::get("/")]
pub async fn find_all(state: web::types::State<AppStateType>, claims: Claims) -> Result<HttpResponse, HttpError> {
	let app_state = state.read().await;
	let user_uuid = claims.get_user_uuid();

	let tasks = match app_state.repositories.task.find_all(user_uuid).await {
		Ok(tasks) => tasks,
		Err(_) => return Err(HttpError::internal_server_error("Failed to find tasks")),
	};

	Ok(HttpResponse::Ok().json(&json!({ "tasks": tasks })))
}

#[web::post("/")]
pub async fn create(
	state: web::types::State<AppStateType>,
	claims: Claims,
	task_input: web::types::Json<TaskCreateInput>,
) -> Result<HttpResponse, HttpError> {
	let app_state = state.read().await;
	let user_uuid = claims.get_user_uuid();

	let task = match app_state.repositories.task.create(task_input.title.clone(), task_input.description.clone(), user_uuid).await
	{
		Ok(task) => task,
		Err(_) => return Err(HttpError::internal_server_error("Failed to create task")),
	};

	Ok(HttpResponse::Created().json(&task))
}

#[web::put("/{cuid}")]
pub async fn update(
	state: web::types::State<AppStateType>,
	claims: Claims,
	cuid: web::types::Path<String>,
	task_input: web::types::Json<TaskUpdateInput>,
) -> Result<HttpResponse, HttpError> {
	let app_state = state.read().await;
	let user_uuid = claims.get_user_uuid();

	let task = match app_state.repositories.task.find_one(cuid.clone()).await {
		Ok(Some(task)) => task,
		Ok(None) => return Err(HttpError::not_found("Task not found")),
		Err(_) => return Err(HttpError::internal_server_error("Failed to find task")),
	};

	if task.user_uuid != user_uuid {
		return Err(HttpError::forbidden("You are not allowed to update this task"));
	}

	let task = match app_state
		.repositories
		.task
		.update(cuid.clone(), task_input.title.clone(), task_input.description.clone(), task_input.done)
		.await
	{
		Ok(task) => task,
		Err(_) => return Err(HttpError::internal_server_error("Failed to update task")),
	};

	Ok(HttpResponse::Ok().json(&task))
}

#[web::delete("/{cuid}")]
pub async fn delete(
	state: web::types::State<AppStateType>,
	claims: Claims,
	cuid: web::types::Path<String>,
) -> Result<HttpResponse, HttpError> {
	let app_state = state.read().await;
	let user_uuid = claims.get_user_uuid();

	let task = match app_state.repositories.task.find_one(cuid.clone()).await {
		Ok(Some(task)) => task,
		Ok(None) => return Err(HttpError::not_found("Task not found")),
		Err(_) => return Err(HttpError::internal_server_error("Failed to find task")),
	};

	if task.user_uuid != user_uuid {
		return Err(HttpError::forbidden("You are not allowed to delete this task"));
	}

	let task = match app_state.repositories.task.delete(cuid.clone()).await {
		Ok(task) => task,
		Err(_) => return Err(HttpError::internal_server_error("Failed to delete task")),
	};

	Ok(HttpResponse::Ok().json(&task))
}

#[web::get("/{cuid}")]
pub async fn find_one(
	state: web::types::State<AppStateType>,
	claims: Claims,
	cuid: web::types::Path<String>,
) -> Result<HttpResponse, HttpError> {
	let app_state = state.read().await;
	let user_uuid = claims.get_user_uuid();

	let task = match app_state.repositories.task.find_one(cuid.clone()).await {
		Ok(Some(task)) => task,
		Ok(None) => return Err(HttpError::not_found("Task not found")),
		Err(_) => return Err(HttpError::internal_server_error("Failed to find task")),
	};

	if task.clone().user_uuid != user_uuid {
		return Err(HttpError::forbidden("You are not allowed to view this task"));
	}

	Ok(HttpResponse::Ok().json(&task))
}

pub fn init(config: &mut web::ServiceConfig) {
	config.service(web::scope("/task").service(find_all).service(create).service(update).service(delete).service(find_one));
}
