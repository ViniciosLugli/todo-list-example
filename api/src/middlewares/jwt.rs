use crate::routes::user::Claims;
use crate::states::app::AppStateType;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web::{self};

pub struct JWTMiddleware<S> {
	service: S,
}

pub struct JWTMiddlewareBuilder {}

impl JWTMiddlewareBuilder {
	pub fn new() -> Self {
		Self {}
	}
}

impl<S> Middleware<S> for JWTMiddlewareBuilder {
	type Service = JWTMiddleware<S>;

	fn create(&self, service: S) -> Self::Service {
		JWTMiddleware { service }
	}
}

impl<S, Err> Service<web::WebRequest<Err>> for JWTMiddleware<S>
where
	S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
	Err: web::ErrorRenderer,
{
	type Response = web::WebResponse;
	type Error = web::Error;

	ntex::forward_poll_ready!(service);

	async fn call(&self, req: web::WebRequest<Err>, ctx: ServiceCtx<'_, Self>) -> Result<Self::Response, Self::Error> {
		let secret;
		{
			let app_state_guard = req.app_state::<AppStateType>().unwrap().read().await;
			secret = app_state_guard.secret;
		}

		if let Some(token) = req.headers().get("Authorization") {
			let token = token.to_str().unwrap().replace("Bearer ", "");
			let token_data =
				decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256));

			if let Ok(token_data) = token_data {
				if token_data.claims.get_expiration() > chrono::Utc::now().timestamp().try_into().unwrap() {
					req.extensions_mut().insert(token_data.claims);
				}
			}
		}

		let res = ctx.call(&self.service, req).await?;

		Ok(res)
	}
}
