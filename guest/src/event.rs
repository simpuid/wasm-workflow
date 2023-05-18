use serde::{Deserialize, Serialize};

pub trait OutgoingEvent {
	fn get_raw(&self) -> anyhow::Result<String>;
}

impl<T: Serialize> OutgoingEvent for T {
	fn get_raw(&self) -> anyhow::Result<String> {
		let string = serde_json::to_string(&self)?;
		Ok(string)
	}
}
