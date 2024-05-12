use std::sync::Arc;

use super::DatabaseClient;
use crate::db::*;
use prisma_client_rust::QueryError;

extern crate bcrypt;
use bcrypt::{hash, verify};

pub type User = user::Data;

const SALT_ROUNDS: u32 = 4;

pub struct UserRepository {
	db_client: DatabaseClient,
}

impl UserRepository {
	pub fn new(db_client: Arc<PrismaClient>) -> Self {
		Self { db_client: DatabaseClient::new(db_client) }
	}

	pub async fn create(&self, name: String, email: String, password: String) -> Result<User, QueryError> {
		let password = hash(password, SALT_ROUNDS).unwrap();

		self.db_client.get_db().user().create(name, email, password, vec![]).exec().await
	}

	pub async fn find_by_credentials(&self, email: String, password: String) -> Result<Option<User>, QueryError> {
		let user = self.db_client.get_db().user().find_first(vec![user::email::equals(email)]).exec().await?;

		match user {
			Some(user) => {
				if verify(password, &user.password).unwrap() {
					Ok(Some(user))
				} else {
					Ok(None)
				}
			}
			None => Ok(None),
		}
	}

	pub async fn find_by_uuid(&self, uuid: String) -> Result<Option<User>, QueryError> {
		self.db_client.get_db().user().find_unique(user::uuid::equals(uuid)).exec().await
	}
}
