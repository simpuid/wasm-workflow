use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::{get, post};
use axum::{Router, Server, ServiceExt};
use wasmtime::{Engine, Module};

use common::{Request, Response};

use crate::db::DbHandler;
use crate::route::{create_handler, update_handler};
use crate::wasm::ModuleCache;

mod db;
mod route;
mod wasm;
// fn main() {
// 	let args: Vec<String> = std::env::args().collect();
// 	let file = args.get(1).unwrap();
// 	let engine = Engine::default();
// 	let module = Module::from_file(&engine, file).unwrap();
//
// 	let mut program = Program::new(&engine, &module).unwrap();
//
// 	let mut cache_state = "".to_string();
// 	let requests = [
// 		Request::Initialization {
// 			parameter: r#" {"initial":0} "#.to_string(),
// 		},
// 		Request::Event {
// 			state: "".to_string(),
// 			event: r#" {"Add":1} "#.to_string(),
// 		},
// 		Request::Event {
// 			state: "".to_string(),
// 			event: r#" {"Add":1} "#.to_string(),
// 		},
// 		Request::Event {
// 			state: "".to_string(),
// 			event: r#" {"Add":2} "#.to_string(),
// 		},
// 		Request::Event {
// 			state: "".to_string(),
// 			event: r#" {"Multiply":3} "#.to_string(),
// 		},
// 	];
// 	for mut rq in requests {
// 		let final_rq = match rq {
// 			Request::Event { event, state: _ } => Request::Event {
// 				event,
// 				state: cache_state.clone(),
// 			},
// 			Request::Initialization { parameter } => Request::Initialization { parameter },
// 		};
// 		let response = program.execute_request(&final_rq).unwrap();
// 		match response {
// 			Response::Snapshot(snapshot) => {
// 				println!("{:?}", &snapshot.state);
// 				cache_state = snapshot.state;
// 			}
// 			Response::Error(err) => println!("{:?}", err),
// 		}
// 	}
// }

pub struct AppState {
	engine: Engine,
	module_cache: ModuleCache,
	db_handler: DbHandler,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let engine = Engine::default();
	let module_cache = ModuleCache::load_directory(&engine, "wasm-files")?;
	let db_handler = DbHandler::load_directory("process-db")?;
	let state = Arc::new(AppState {
		engine,
		module_cache,
		db_handler,
	});
	let app = Router::new()
		.route("/create", post(create_handler))
		.route("/update", post(update_handler))
		.with_state(state);
	let address = SocketAddr::from(([127, 0, 0, 1], 3000));
	Server::bind(&address)
		.serve(app.into_make_service())
		.await?;
	Ok(())
}
