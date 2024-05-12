extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[allow(warnings, unused)]
mod db;
mod error;
mod middlewares;
mod repositories;
mod routes;
mod states;
mod utils;

use db::*;
use ntex::{
	http,
	web::{self, middleware, App},
};
use ntex_cors::Cors;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::states::app::AppState;

#[ntex::main]
async fn main() -> std::io::Result<()> {
	dotenvy::dotenv().ok();
	pretty_env_logger::init();
	info!("Starting server...");
	let client = PrismaClient::_builder().build().await.unwrap();
	info!("Connected to database!");

	println!("Running migrations...");

	#[cfg(debug_assertions)]
	client._db_push().await.unwrap();

	println!("Migrating database...");

	#[cfg(not(debug_assertions))]
	client._migrate_deploy().await?;

	info!("Database schema is up to date!");

	let client = Arc::new(client);
	let repositories = repositories::Repositories::new(client.clone());

	let state = Arc::new(RwLock::new(AppState::new(client, repositories)));
	info!("Server is running on http://0.0.0.0:3000");
	web::server(move || {
		App::new()
			.state(state.clone())
			.wrap(middleware::Logger::default())
			.wrap(
				Cors::new()
					.allowed_origin("*")
					.allowed_methods(vec!["GET", "POST", "DELETE"])
					.allowed_headers(vec![http::header::ACCEPT])
					.allowed_header(http::header::CONTENT_TYPE)
					.max_age(3600)
					.finish(),
			)
			.configure(routes::user::init)
			.wrap(middlewares::jwt::JWTMiddlewareBuilder::new())
			.configure(routes::task::init)
	})
	.bind("0.0.0.0:3000")?
	.run()
	.await
}
