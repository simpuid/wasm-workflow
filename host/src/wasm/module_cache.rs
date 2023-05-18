use std::collections::HashMap;
use std::ffi::OsStr;

use walkdir::{DirEntry, WalkDir};
use wasmtime::{Engine, Module};

pub struct ModuleCache {
	map: HashMap<String, Module>,
}

impl ModuleCache {
	pub fn load_directory(engine: &Engine, directory: &str) -> anyhow::Result<ModuleCache> {
		let mut map = HashMap::new();
		for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
			if let Some((filename, path)) = extract_wasm_file_name(entry) {
				let module = Module::from_file(engine, path)?;
				map.insert(filename, module);
			}
		}
		Ok(ModuleCache { map })
	}

	pub fn get_module(&self, id: &str) -> Option<&Module> {
		self.map.get(id)
	}
}

fn extract_wasm_file_name(entry: DirEntry) -> Option<(String, String)> {
	let metadata = entry.metadata().ok()?;
	if !metadata.is_file() {
		return None;
	}
	if Some(OsStr::new("wasm")) != entry.path().extension() {
		return None;
	}
	let filename = entry.file_name().to_str()?.to_string();
	let path = entry.path().to_str()?.to_string();
	Some((filename, path))
}
