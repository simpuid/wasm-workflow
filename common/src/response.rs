use crate::Operation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
	Error(String),
	Snapshot(Snapshot),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
	pub operations: Vec<Operation>,
	pub state: String,
}
