use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::actions::Actions;
use crate::state::{EventStatus, State};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Store<T> {
	state: Box<T>,
}

impl<T: State> Store<T> {
	pub fn new(state: T) -> Store<T> {
		Store {
			state: Box::new(state),
		}
	}
}

impl<T: State> Serialize for Store<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		return self.state.serialize(serializer);
	}
}

impl<'de, T: State> Deserialize<'de> for Store<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let state = T::deserialize(deserializer)?;
		return Ok(Store::new(state));
	}
}

impl<T: State> BaseStore for Store<T> {
	fn process(&mut self, event: &str) -> (EventStatus, Actions) {
		for store in self.state.inner_store() {
			let (event_status, actions) = store.process(event);
			if event_status == EventStatus::Consumed {
				return (event_status, actions);
			}
		}
		if let Ok(event) = serde_json::from_str::<T::Event>(event) {
			let (new_state, event_status, actions) = T::process(self.state.as_ref().clone(), event);
			*self.state = new_state;
			return (event_status, actions);
		}
		(EventStatus::Dropped, Actions::new())
	}

	fn update(&mut self) -> (StateStatus, Actions) {
		let mut actions = Actions::new();
		let mut state_status = StateStatus::Same;
		for store in self.state.inner_store() {
			let (new_state_status, new_actions) = store.update();
			if new_state_status == StateStatus::Changed {
				state_status = StateStatus::Changed;
			}
			actions = actions.merge(new_actions);
		}
		let (new_state, new_actions) = T::update(self.state.as_ref().clone());
		if &new_state != self.state.as_ref() {
			state_status = StateStatus::Changed;
			*self.state = new_state;
		}
		actions = actions.merge(new_actions);
		(state_status, actions)
	}
}

pub trait BaseStore {
	fn process(&mut self, event: &str) -> (EventStatus, Actions);
	fn update(&mut self) -> (StateStatus, Actions);
}

#[derive(Eq, PartialEq)]
pub enum StateStatus {
	Changed,
	Same,
}
