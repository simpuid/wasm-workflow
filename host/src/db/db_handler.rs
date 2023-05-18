use sled::Db;

pub struct DbHandler {
	db: Db,
}
impl DbHandler {
	pub fn load_directory(path: &str) -> anyhow::Result<DbHandler> {
		Ok(DbHandler {
			db: sled::open(path)?,
		})
	}

	pub fn insert(&self, wasm: &str, process_id: &str, data: &str) -> anyhow::Result<()> {
		self.db.insert(
			form_key(wasm, process_id).as_str().as_bytes(),
			data.as_bytes(),
		)?;
		Ok(())
	}

	pub fn get(&self, wasm: &str, process_id: &str) -> anyhow::Result<String> {
		let entry = self
			.db
			.get(form_key(wasm, process_id).as_str().as_bytes())?
			.ok_or(anyhow::Error::msg("process_not_found"))?;
		Ok(std::str::from_utf8(entry.as_ref())?.to_string())
	}
}

fn form_key(wasm: &str, process_id: &str) -> String {
	format!("{}::{}", wasm, process_id)
}
