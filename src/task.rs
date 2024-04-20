use serde::{Deserialize, Serialize};

use crate::user::{PublicUser, User};

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
	pub id: usize,
	pub content: String,
	pub completed: bool,
	pub owner: PublicUser,
}

impl Task {
	pub fn new(id: usize, content: String, owner: &User) -> Task {
		Task { id, content, completed: false, owner: PublicUser::from(owner) }
	}
}
