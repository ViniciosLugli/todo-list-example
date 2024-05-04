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

#[cfg(test)]
mod task_tests {
	use super::*;
	use crate::user::User;

	#[test]
	fn test_create_task() {
		let user = User::new("testuser", "password");
		let task = Task::new(1, "Do something".to_string(), &user);
		assert_eq!(task.content, "Do something");
		assert!(!task.completed);
	}
}
