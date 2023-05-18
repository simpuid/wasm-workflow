use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
	Event(String),
	Info(String),
	Error(String),
}
