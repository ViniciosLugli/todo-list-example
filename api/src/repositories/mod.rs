use crate::db::*;
use std::sync::Arc;

pub struct DatabaseClient {
	db: Arc<PrismaClient>,
}

impl DatabaseClient {
	pub fn new(db: Arc<PrismaClient>) -> Self {
		Self { db }
	}

	pub fn get_db(&self) -> Arc<PrismaClient> {
		self.db.clone()
	}
}

mod task;
mod user;

pub struct Repositories {
	pub user: user::UserRepository,
	pub task: task::TaskRepository,
}

impl Repositories {
	pub fn new(db: Arc<PrismaClient>) -> Self {
		Self { user: user::UserRepository::new(db.clone()), task: task::TaskRepository::new(db.clone()) }
	}
}
