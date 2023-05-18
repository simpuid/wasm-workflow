use serde::Serialize;

use common::Operation;

use crate::event::OutgoingEvent;

pub struct Actions {
	logs: Vec<ActionLog>,
}

impl Actions {
	pub fn new() -> Actions {
		Actions { logs: Vec::new() }
	}

	pub fn info<T: Into<String>>(mut self, string: T) -> Self {
		self.logs.push(ActionLog::Info(string.into()));
		self
	}

	pub fn event<T: OutgoingEvent + 'static>(mut self, event: T) -> Self {
		self.logs.push(ActionLog::Event(Box::new(event)));
		self
	}

	pub fn merge(mut self, mut other: Actions) -> Self {
		self.logs.append(&mut other.logs);
		self
	}

	pub fn build(self) -> anyhow::Result<Vec<Operation>> {
		let mut operations = Vec::new();
		for action_log in self.logs {
			operations.push(match action_log {
				ActionLog::Event(e) => Operation::Event(e.get_raw()?),
				ActionLog::Info(s) => Operation::Info(s),
			});
		}
		Ok(operations)
	}
}

enum ActionLog {
	Event(Box<dyn OutgoingEvent>),
	Info(String),
}
