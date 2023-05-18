use common::{Request, Response, Snapshot};

use crate::actions;
use crate::actions::Actions;
use crate::state::{EventStatus, State};
use crate::store::{BaseStore, StateStatus, Store};

pub trait Executor {
	type RootState: State;
	const UPDATE_LIMIT: usize = 50;

	fn execute(input: &str) -> String {
		match Self::raw_execute(input) {
			Ok(response) => response,
			Err(error) => error.to_string(),
		}
	}

	fn raw_execute(input: &str) -> anyhow::Result<String> {
		let request = serde_json::from_str(input)?;
		let response = match Self::serialized_execute(request) {
			Ok(snapshot) => serde_json::to_string(&Response::Snapshot(snapshot))?,
			Err(error) => serde_json::to_string(&Response::Error(error.to_string()))?,
		};
		Ok(response)
	}

	fn serialized_execute(request: Request) -> anyhow::Result<Snapshot> {
		let snapshot = match request {
			Request::Initialization { parameter } => Self::execute_initialization(parameter)?,
			Request::Event { state, event } => Self::execute_event(state, event)?,
		};
		Ok(snapshot)
	}

	fn execute_initialization(parameter: String) -> anyhow::Result<Snapshot> {
		let parameter = serde_json::from_str(parameter.as_str())?;
		let (state, actions) = Self::RootState::entry(parameter);
		let snapshot = Snapshot {
			operations: actions.build()?,
			state: serde_json::to_string(&state)?,
		};
		Ok(snapshot)
	}

	fn execute_event(state: String, event: String) -> anyhow::Result<Snapshot> {
		let mut store: Store<Self::RootState> = Store::new(serde_json::from_str(state.as_str())?);
		let (mut event_status, mut actions) = store.process(event.as_str());
		if event_status == EventStatus::Consumed {
			for _ in 0..Self::UPDATE_LIMIT {
				let (state_status, new_actions) = store.update();
				actions = actions.merge(new_actions);
				if state_status == StateStatus::Same {
					return Self::snapshot(store, actions);
				}
			}
			return Err(anyhow::Error::msg("update_limit_exceeded"));
		}
		return Self::snapshot(store, actions);
	}

	fn snapshot(store: Store<Self::RootState>, actions: Actions) -> anyhow::Result<Snapshot> {
		let snapshot = Snapshot {
			operations: actions.build()?,
			state: serde_json::to_string(&store)?,
		};
		return Ok(snapshot);
	}
}
