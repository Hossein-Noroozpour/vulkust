use super::super::super::render::shader::manager::Manager as ShaderManager;
use super::super::super::io::read::Read;
use super::super::resource::manager::Manager as ResourceManager;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Manager<File> where File: Read {
	pub shader_manager: ShaderManager<File>,
}

impl<File> Manager<File> where File: Read {
	pub fn new(file: File) -> Self {
		let file = Arc::new(Mutex::new(file));
		let mut shader_manager = ShaderManager::new(file);
		shader_manager.read_tabale();
		Manager {
			shader_manager: shader_manager,
		}
	}
}
