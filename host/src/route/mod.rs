use std::error::Error;
use std::fmt::Display;

use axum::http::StatusCode;
use serde::Serialize;

pub use create::create_handler;
pub use update::update_handler;
mod create;
mod update;

#[derive(Serialize)]
#[serde(untagged)]
pub enum HandlerResponse<T> {
	Value(T),
	Error { status_code: u16, error: String },
}

impl<T: Serialize> HandlerResponse<T> {
	pub fn from_result<E: Display>(result: Result<T, E>) -> HandlerResponse<T> {
		match result {
			Ok(t) => HandlerResponse::Value(t),
			Err(e) => HandlerResponse::Error {
				status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
				error: e.to_string(),
			},
		}
	}
}
