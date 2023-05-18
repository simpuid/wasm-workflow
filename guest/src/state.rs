use serde::{Deserialize, Serialize};

use crate::actions::Actions;
use crate::store::BaseStore;

pub trait State: Eq + Clone + Serialize + for<'a> Deserialize<'a> {
	type Event: Serialize + for<'a> Deserialize<'a>;
	type Parameter: for<'a> Deserialize<'a>;
	fn entry(parameter: Self::Parameter) -> (Self, Actions);
	fn process(self, event: Self::Event) -> (Self, EventStatus, Actions);
	fn update(self) -> (Self, Actions);
	fn inner_store(&mut self) -> Vec<&mut dyn BaseStore>;
}

#[derive(Eq, PartialEq)]
pub enum EventStatus {
	Consumed,
	Dropped,
}
