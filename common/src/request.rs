use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
	Initialization { parameter: String },
	Event { state: String, event: String },
}
