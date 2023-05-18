use std::os::linux::raw::stat;
use std::sync::Arc;

use axum::extract::State;
use axum::routing::any;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use uuid::Uuid;
use wasmtime::Engine;

use common::{Operation, Request, Response};

use crate::db::DbHandler;
use crate::route::HandlerResponse;
use crate::wasm::{ModuleCache, Program};
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateRequest {
	wasm: String,
	parameter: Map<String, Value>,
}

#[derive(Serialize)]
pub struct CreateResponse {
	wasm: String,
	process_id: String,
	state: Map<String, Value>,
	operations: Vec<Operation>,
}

pub async fn create_handler(
	State(state): State<Arc<AppState>>,
	request: Json<CreateRequest>,
) -> Json<HandlerResponse<CreateResponse>> {
	HandlerResponse::from_result(create(request.0, &state)).into()
}

fn create(request: CreateRequest, app_state: &AppState) -> anyhow::Result<CreateResponse> {
	let module = app_state
		.module_cache
		.get_module(request.wasm.as_str())
		.ok_or(anyhow::Error::msg("module_not_found"))?;
	let mut program = Program::new(&app_state.engine, &module)?;
	let parameter = serde_json::to_string(&request.parameter)?;
	let program_request = Request::Initialization { parameter };
	let response = program.execute_request(&program_request)?;
	let snapshot = match response {
		Response::Error(e) => return Err(anyhow::Error::msg(e)),
		Response::Snapshot(s) => s,
	};
	let process_id = Uuid::new_v4().to_string();
	app_state.db_handler.insert(
		request.wasm.as_str(),
		process_id.as_str(),
		snapshot.state.as_str(),
	)?;
	Ok(CreateResponse {
		operations: snapshot.operations,
		wasm: request.wasm,
		process_id,
		state: serde_json::from_str(snapshot.state.as_str())?,
	})
}
