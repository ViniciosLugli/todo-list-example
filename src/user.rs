use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
	pub username: String,
	password_hash: String,
}

impl User {
	pub fn new(username: &str, password: &str) -> User {
		let password_hash = hash(password, 4).unwrap();
		User { username: username.to_string(), password_hash }
	}

	pub fn authenticate(&self, password: &str) -> bool {
		verify(password, &self.password_hash).unwrap()
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PublicUser {
	pub username: String,
}

impl From<&User> for PublicUser {
	fn from(user: &User) -> Self {
		PublicUser { username: user.username.clone() }
	}
}
