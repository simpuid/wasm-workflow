use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use common::{Operation, Request, Response};

use crate::route::HandlerResponse;
use crate::wasm::Program;
use crate::AppState;

#[derive(Deserialize)]
pub struct UpdateRequest {
	wasm: String,
	process_id: String,
	event: Map<String, Value>,
}

#[derive(Serialize)]
pub struct UpdateResponse {
	wasm: String,
	process_id: String,
	state: Map<String, Value>,
	operations: Vec<Operation>,
}

pub async fn update_handler(
	State(state): State<Arc<AppState>>,
	request: Json<UpdateRequest>,
) -> Json<HandlerResponse<UpdateResponse>> {
	HandlerResponse::from_result(update(request.0, &state)).into()
}

fn update(request: UpdateRequest, app_state: &AppState) -> anyhow::Result<UpdateResponse> {
	let module = app_state
		.module_cache
		.get_module(request.wasm.as_str())
		.ok_or(anyhow::Error::msg("module_not_found"))?;
	let state = app_state
		.db_handler
		.get(request.wasm.as_str(), request.process_id.as_str())?;

	let mut program = Program::new(&app_state.engine, &module)?;
	let event = serde_json::to_string(&request.event)?;
	let program_request = Request::Event { state, event };
	let response = program.execute_request(&program_request)?;
	let snapshot = match response {
		Response::Error(e) => return Err(anyhow::Error::msg(e)),
		Response::Snapshot(s) => s,
	};
	app_state.db_handler.insert(
		request.wasm.as_str(),
		request.process_id.as_str(),
		snapshot.state.as_str(),
	)?;
	Ok(UpdateResponse {
		operations: snapshot.operations,
		wasm: request.wasm,
		process_id: request.process_id,
		state: serde_json::from_str(snapshot.state.as_str())?,
	})
}
