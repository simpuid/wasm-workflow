use wasmtime::{Engine, Module};

use common::{Request, Response};

use crate::program::Program;

mod program;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let file = args.get(1).unwrap();
	let engine = Engine::default();
	let module = Module::from_file(&engine, file).unwrap();

	let mut program = Program::new(&engine, &module).unwrap();

	let mut cache_state = "".to_string();
	let requests = [
		Request::Initialization {
			parameter: r#" {"initial":0} "#.to_string(),
		},
		Request::Event {
			state: "".to_string(),
			event: r#" {"Add":1} "#.to_string(),
		},
		Request::Event {
			state: "".to_string(),
			event: r#" {"Add":1} "#.to_string(),
		},
		Request::Event {
			state: "".to_string(),
			event: r#" {"Add":2} "#.to_string(),
		},
		Request::Event {
			state: "".to_string(),
			event: r#" {"Multiply":3} "#.to_string(),
		},
	];
	for mut rq in requests {
		let final_rq = match rq {
			Request::Event { event, state: _ } => Request::Event {
				event,
				state: cache_state.clone(),
			},
			Request::Initialization { parameter } => Request::Initialization { parameter },
		};
		let response = program.execute_request(&final_rq).unwrap();
		match response {
			Response::Snapshot(snapshot) => {
				println!("{:?}", &snapshot.state);
				cache_state = snapshot.state;
			}
			Response::Error(err) => println!("{:?}", err),
		}
	}
}
