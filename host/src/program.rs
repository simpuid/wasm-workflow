use std::i32;

use wasmtime::{Engine, Instance, Memory, Module, Store, TypedFunc};

use common::{Request, Response};

const MEMORY_EXPORT_NAME: &str = "memory";
const ALLOC_FUNC_EXPORT_NAME: &str = "alloc";
const DEALLOC_FUNC_EXPORT_NAME: &str = "dealloc";
const APPLY_FUNC_EXPORT_NAME: &str = "apply";

pub struct Program {
	store: Store<()>,
	memory: Memory,
	alloc_function: TypedFunc<i32, i32>,
	dealloc_function: TypedFunc<(i32, i32), ()>,
	apply_function: TypedFunc<(i32, i32, i32, i32), ()>,
}

impl Program {
	pub fn new(engine: &Engine, module: &Module) -> anyhow::Result<Program> {
		let mut store = Store::new(&engine, ());
		let instance = Instance::new(&mut store, &module, &[])?;
		let memory = instance
			.get_memory(&mut store, MEMORY_EXPORT_NAME)
			.ok_or(anyhow::Error::msg("error_accessing_memory"))?;
		let alloc_function =
			instance.get_typed_func::<i32, i32>(&mut store, ALLOC_FUNC_EXPORT_NAME)?;
		let dealloc_function =
			instance.get_typed_func::<(i32, i32), ()>(&mut store, DEALLOC_FUNC_EXPORT_NAME)?;
		let apply_function = instance
			.get_typed_func::<(i32, i32, i32, i32), ()>(&mut store, APPLY_FUNC_EXPORT_NAME)?;
		Ok(Program {
			store,
			memory,
			alloc_function,
			dealloc_function,
			apply_function,
		})
	}

	fn execute_alloc(&mut self, size: usize) -> anyhow::Result<(i32, i32)> {
		let size = i32::try_from(size)?;
		let pointer = self.alloc_function.call(&mut self.store, size)?;
		Ok((pointer, size))
	}

	fn execute_dealloc(&mut self, pointer: i32, size: i32) -> anyhow::Result<()> {
		self.dealloc_function.call(&mut self.store, (pointer, size))
	}

	fn execute_apply(
		&mut self,
		input_pointer: i32,
		input_size: i32,
		output_pointer_pointer: i32,
		output_size_pointer: i32,
	) -> anyhow::Result<()> {
		self.apply_function.call(
			&mut self.store,
			(
				input_pointer,
				input_size,
				output_pointer_pointer,
				output_size_pointer,
			),
		)
	}

	fn apply(&mut self, input: &str) -> anyhow::Result<String> {
		let input_slice = input.as_bytes();
		let (input_pointer, input_size) = self.execute_alloc(input_slice.len())?;
		unsafe {
			let pointer = self
				.memory
				.data_ptr(&self.store)
				.offset(isize::try_from(input_pointer)?);
			pointer.copy_from(input_slice.as_ptr(), input_slice.len());
		}
		let (parameter_pointer, parameter_size) = self.execute_alloc(8)?;
		self.execute_apply(
			input_pointer,
			input_size,
			parameter_pointer,
			parameter_pointer + i32::try_from(std::mem::size_of::<i32>())?,
		)?;
		let (output_pointer, output_size) = unsafe {
			let pointer = self
				.memory
				.data_ptr(&self.store)
				.offset(isize::try_from(parameter_pointer)?);
			let mut output_pointer = i32::default();
			pointer.copy_to(
				&mut output_pointer as *mut i32 as *mut u8,
				std::mem::size_of::<i32>(),
			);
			let mut output_size = i32::default();
			let pointer = self.memory.data_ptr(&self.store).offset(
				isize::try_from(parameter_pointer)? + isize::try_from(std::mem::size_of::<i32>())?,
			);
			pointer.copy_to(
				&mut output_size as *mut i32 as *mut u8,
				std::mem::size_of::<i32>(),
			);
			(output_pointer, output_size)
		};
		self.execute_dealloc(parameter_pointer, parameter_size)?;
		let output = unsafe {
			let pointer = self
				.memory
				.data_ptr(&mut self.store)
				.offset(isize::try_from(output_pointer)?);
			let slice = std::slice::from_raw_parts(pointer, usize::try_from(output_size)?);
			String::from_utf8_unchecked(Vec::from(slice))
		};
		self.execute_dealloc(output_pointer, output_size)?;
		Ok(output)
	}

	pub fn execute_request(&mut self, request: &Request) -> anyhow::Result<Response> {
		let request_string = serde_json::to_string(request)?;
		let response_string = self.apply(request_string.as_str())?;
		let response = serde_json::from_str(response_string.as_str())?;
		Ok(response)
	}
}
