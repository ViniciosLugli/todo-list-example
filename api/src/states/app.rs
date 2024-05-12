use crate::{db, repositories};
use db::*;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
	pub db: Arc<PrismaClient>,
	pub repositories: repositories::Repositories,
	pub secret: &'static str,
}

impl AppState {
	pub fn new(db: Arc<PrismaClient>, repositories: repositories::Repositories) -> Self {
		Self { db, repositories, secret: option_env!("JWT_SECRET").unwrap_or("secret") }
	}
}

pub type AppStateType = Arc<RwLock<AppState>>;
