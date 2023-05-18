use serde::{Deserialize, Serialize};

use guest::{Actions, BaseStore, EventStatus, Executor, GuestInterface, State};

#[no_mangle]
extern "C" fn alloc(size: i32) -> i32 {
	WorkflowExecutor::alloc(size)
}

#[no_mangle]
extern "C" fn dealloc(ptr: i32, size: i32) {
	WorkflowExecutor::dealloc(ptr, size)
}

#[no_mangle]
extern "C" fn apply(in_ptr: i32, in_size: i32, out_ptr: i32, out_size: i32) {
	WorkflowExecutor::apply(in_ptr, in_size, out_ptr, out_size);
}

struct WorkflowExecutor;

impl Executor for WorkflowExecutor {
	type RootState = WorkflowState;
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
struct WorkflowState {
	accumulator: isize,
}

#[derive(Serialize, Deserialize)]
enum WorkflowStateEvent {
	Add(isize),
	Subtract(isize),
	Multiply(isize),
}

#[derive(Serialize, Deserialize)]
struct WorkflowStateParameter {
	initial: isize,
}

impl State for WorkflowState {
	type Event = WorkflowStateEvent;
	type Parameter = WorkflowStateParameter;

	fn entry(parameter: Self::Parameter) -> (Self, Actions) {
		(
			WorkflowState {
				accumulator: parameter.initial,
			},
			Actions::new(),
		)
	}

	fn process(self, event: Self::Event) -> (Self, EventStatus, Actions) {
		match event {
			WorkflowStateEvent::Add(n) => (
				WorkflowState {
					accumulator: self.accumulator + n.clone(),
				},
				EventStatus::Consumed,
				Actions::new(),
			),
			WorkflowStateEvent::Subtract(n) => (
				WorkflowState {
					accumulator: self.accumulator - n,
				},
				EventStatus::Consumed,
				Actions::new(),
			),
			WorkflowStateEvent::Multiply(n) => (
				WorkflowState {
					accumulator: self.accumulator * n,
				},
				EventStatus::Consumed,
				Actions::new(),
			),
		}
	}

	fn update(self) -> (Self, Actions) {
		(self, Actions::new())
	}

	fn inner_store(&mut self) -> Vec<&mut dyn BaseStore> {
		vec![]
	}
}
