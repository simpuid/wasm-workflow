use std::alloc::Layout;

pub use actions::Actions;
pub use executor::Executor;
pub use guest_interface::GuestInterface;
pub use state::{EventStatus, State};
pub use store::{BaseStore, Store};

mod actions;
mod event;
mod executor;
mod guest_interface;
mod state;
mod store;

#[cfg(test)]
mod tests {
	use serde::{Deserialize, Serialize};

	use crate::actions::Actions;
	use crate::executor::Executor;
	use crate::state;
	use crate::state::{EventStatus, State};
	use crate::store::{BaseStore, Store};

	#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
	enum SuperState {
		None(String),
		Single(String, Store<MyState>),
		Double(String, Store<MyState>, Store<MyState>),
	}

	impl State for SuperState {
		type Event = String;
		type Parameter = ();

		fn entry(parameter: Self::Parameter) -> (Self, Actions) {
			(SuperState::None("s".to_string()), Actions::new())
		}

		fn process(self, event: Self::Event) -> (Self, EventStatus, Actions) {
			println!("process: {:?}", &self);

			match self {
				SuperState::None(s) => (
					SuperState::Single(s + event.as_str(), Store::new(MyState::A(0))),
					EventStatus::Consumed,
					Actions::new(),
				),
				SuperState::Single(s, m1) => (
					SuperState::Double(s + event.as_str(), Store::new(MyState::A(0)), m1),
					EventStatus::Consumed,
					Actions::new(),
				),
				SuperState::Double(s, m1, m2) => (
					SuperState::None(s + event.as_str()),
					EventStatus::Consumed,
					Actions::new(),
				),
			}
		}

		fn update(self) -> (Self, Actions) {
			println!("update {:?}", &self);

			(self, Actions::new())
		}

		fn inner_store(&mut self) -> Vec<&mut dyn BaseStore> {
			match self {
				SuperState::None(_) => vec![],
				SuperState::Single(_, m1) => vec![m1],
				SuperState::Double(_, m1, m2) => vec![m1, m2],
			}
		}
	}

	#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
	enum MyState {
		A(i32),
		B(i32),
		C(i32),
		D(i32),
	}

	impl State for MyState {
		type Event = i32;
		type Parameter = ();

		fn entry(parameter: Self::Parameter) -> (Self, Actions) {
			(MyState::A(0), Actions::new())
		}

		fn process(self, event: Self::Event) -> (Self, EventStatus, Actions) {
			println!("process: {:?}", &self);

			match self {
				MyState::A(e) => (
					MyState::B(e + event),
					EventStatus::Consumed,
					Actions::new().event(1),
				),
				MyState::B(e) => (MyState::C(e + event), EventStatus::Consumed, Actions::new()),
				MyState::C(e) => (
					MyState::D(e + event),
					EventStatus::Consumed,
					Actions::new().event(1),
				),
				MyState::D(e) => (MyState::A(e + event), EventStatus::Consumed, Actions::new()),
			}
		}

		fn update(self) -> (Self, Actions) {
			println!("update {:?}", &self);

			(self, Actions::new())
		}

		fn inner_store(&mut self) -> Vec<&mut dyn BaseStore> {
			vec![]
		}
	}

	struct MyExecutor;
	impl Executor for MyExecutor {
		type RootState = SuperState;
	}

	#[test]
	fn test_store() {
		let mut state = MyExecutor::execute_initialization(serde_json::to_string(&()).unwrap())
			.unwrap()
			.state;
		println!("init: {}", state);

		let inputs = [
			"\"a\"", "1", "1", "1", "1", "1", "1", "\"b\"", "1", "1", "1", "1", "1", "1", "1",
			"\"c\"", "1", "1", "1", "1", "1",
		];
		for s in inputs {
			state = MyExecutor::execute_event(state, s.to_string())
				.unwrap()
				.state;
			println!("state: {}\n", state);
		}
	}
}
