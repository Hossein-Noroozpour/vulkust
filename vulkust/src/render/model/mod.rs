pub mod manager;

use std::sync::Arc;
use std::cell::RefCell;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::Mat4x4;
use super::super::system::os::OsApplication;
use super::super::system::file::File;
use super::mesh::Mesh;

pub trait Model {}

pub struct StaticModel {
    pub draw_mesh: Mesh,
    pub children: Vec<Box<Model>>,
}

impl StaticModel {
    pub fn new<CoreApp>(file: &mut File, os_app: &mut OsApplication<CoreApp>) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let mesh = Mesh::new(file, os_app);
        logf!("Unimplmented");
    }
}

impl Model for StaticModel {}

pub struct DynamicModel {
    pub occ_mesh: Mesh,
    pub children: Vec<Box<Model>>,
}

impl DynamicModel {
    pub fn new<CoreApp>(file: &mut File, os_app: &mut OsApplication<CoreApp>) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        logf!("Unimplmented");
    }
}

impl Model for DynamicModel {}

pub struct CopyModel {
    pub t: Mat4x4<f32>,
    pub sm: Arc<RefCell<Model>>,
}

impl CopyModel {
    pub fn new<CoreApp>(file: &mut File, os_app: &mut OsApplication<CoreApp>) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let t = Mat4x4::new_from_file(file);
        let id = file.read_id();
        CopyModel {
            t: t,
            sm: unsafe { (*os_app).asset_manager.get_model(id, os_app) },
        }
    }
}

impl Model for CopyModel {}

pub fn read_model<CoreApp>(
    file: &mut File,
    os_app: &mut OsApplication<CoreApp>,
) -> Arc<RefCell<Model>>
where
    CoreApp: ApplicationTrait,
{
    let dyn = file.read_bool();
    let is_copy = file.read_bool();
    if is_copy {
        return Arc::new(RefCell::new(CopyModel::new(file, os_app)));
    }
    if dyn {
        return Arc::new(RefCell::new(DynamicModel::new(file, os_app)));
    }
    return Arc::new(RefCell::new(StaticModel::new(file, os_app)));
}
