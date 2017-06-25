use super::super::super::render::shader::manager::Manager as ShaderManager;
use super::super::super::render::shader::ShaderTrait;
use super::super::super::render::texture::manager::Manager as TextureManager;
use super::super::super::render::texture::TextureTrait;
use super::super::super::system::file::File;
use super::super::super::system::os::OsApplication;
use super::super::application::ApplicationTrait;
use std::sync::{Arc};

pub struct Manager {
	pub file: File,
	pub shader_manager: ShaderManager,
	pub texture_manager: TextureManager,
}

impl Manager {
	pub fn new(file: File) -> Self {
		Manager {
			file: file,
			shader_manager: ShaderManager::new(),
			texture_manager: TextureManager::new(),
		}
	}

	pub fn initialize(&mut self) {
		self.shader_manager.read_tabale(&mut self.file);
		self.texture_manager.read_tabale(&mut self.file);
	}

	pub fn get_shader<CoreApp>(&mut self, id: u64, os_app: *mut OsApplication<CoreApp>) ->
			Arc<ShaderTrait> where CoreApp: ApplicationTrait {
		self.shader_manager.get(id, &mut self.file, os_app)
	}

	pub fn get_texture<CoreApp>(&mut self, id: u64, os_app: *mut OsApplication<CoreApp>) ->
			Arc<TextureTrait> where CoreApp: ApplicationTrait {
		self.texture_manager.get(id, &mut self.file, os_app)
	}
}
