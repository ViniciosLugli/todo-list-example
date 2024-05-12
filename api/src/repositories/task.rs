use std::sync::Arc;

use super::DatabaseClient;
use crate::{db::*, utils::datetime::db_now_datetime};
use prisma_client_rust::QueryError;

pub type Task = task::Data;

pub struct TaskRepository {
	db_client: DatabaseClient,
}

impl TaskRepository {
	pub fn new(db_client: Arc<PrismaClient>) -> Self {
		Self { db_client: DatabaseClient::new(db_client) }
	}

	pub async fn create(&self, title: String, description: String, user_uuid: String) -> Result<Task, QueryError> {
		self.db_client.get_db().task().create(title, description, user::uuid::equals(user_uuid), vec![]).exec().await
	}

	pub async fn find_all(&self, user_uuid: String) -> Result<Vec<Task>, QueryError> {
		self.db_client.get_db().task().find_many(vec![task::user::is(vec![user::uuid::equals(user_uuid)])]).exec().await
	}

	pub async fn find_one(&self, cuid: String) -> Result<Option<Task>, QueryError> {
		self.db_client.get_db().task().find_unique(task::cuid::equals(cuid)).exec().await
	}

	pub async fn update(&self, cuid: String, title: String, description: String, done: bool) -> Result<Task, QueryError> {
		self.db_client
			.get_db()
			.task()
			.update(
				task::cuid::equals(cuid),
				vec![
					task::title::set(title),
					task::description::set(description),
					task::done::set(done),
					task::updated_at::set(db_now_datetime()),
				],
			)
			.exec()
			.await
	}

	pub async fn delete(&self, cuid: String) -> Result<Task, QueryError> {
		self.db_client.get_db().task().delete(task::cuid::equals(cuid)).exec().await
	}
}
