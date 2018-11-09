use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, Real};
use super::camera::Camera;
use super::command::Buffer as CmdBuffer;
use super::engine::Engine;
use super::font::Font;
use super::material::Material;
use super::mesh::{Base as MeshBase, Mesh};
use super::model::{Base as ModelBase, DefaultModel, Model, Uniform};
use super::object::{Object, Transferable};
use super::scene::Scene;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use math;
use rusttype::{point, Scale};

pub trait Widget: Model {}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    pub model_base: ModelBase,
    pub sensitive: bool,
}

impl CoreObject for Base {
    fn get_id(&self) -> Id {
        self.model_base.get_id()
    }
}

impl Object for Base {
    fn get_name(&self) -> Option<String> {
        self.model_base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.model_base.set_name(name);
        vxunimplemented!(); //it must update corresponding manager
    }

    fn disable_rendering(&mut self) {
        self.model_base.disable_rendering();
    }

    fn enable_rendering(&mut self) {
        self.model_base.enable_rendering()
    }

    fn is_rendarable(&self) -> bool {
        return self.model_base.is_rendarable();
    }
}

impl Transferable for Base {
    fn set_orientation(&mut self, q: &math::Quaternion<Real>) {
        self.model_base.set_orientation(q);
    }

    fn set_location(&mut self, l: &math::Vector3<Real>) {
        self.model_base.set_location(l);
    }

    fn get_location(&self) -> math::Vector3<Real> {
        return self.model_base.get_location();
    }

    fn move_local_z(&mut self, d: Real) {
        self.model_base.move_local_z(d);
    }

    fn move_local_x(&mut self, d: Real) {
        self.model_base.move_local_x(d);
    }

    fn rotate_local_x(&mut self, d: Real) {
        self.model_base.rotate_local_x(d);
    }

    fn rotate_global_z(&mut self, d: Real) {
        self.model_base.rotate_global_z(d);
    }
}

impl Model for Base {
    fn update(&mut self, scene: &Scene, camera: &Camera, frame_number: usize) {
        self.model_base.update(scene, camera, frame_number);
    }

    fn add_mesh(&mut self, mesh: Arc<RwLock<Mesh>>) {
        self.model_base.add_mesh(mesh);
    }

    fn get_meshes(&self) -> &BTreeMap<Id, Arc<RwLock<Mesh>>> {
        return self.model_base.get_meshes();
    }

    fn clear_meshes(&mut self) {
        self.model_base.clear_meshes();
    }

    fn bring_all_child_models(&self) -> Vec<(Id, Arc<RwLock<Model>>)> {
        return self.model_base.bring_all_child_models();
    }

    fn has_shadow(&self) -> bool {
        return false;
    }

    fn get_occlusion_culling_radius(&self) -> Real {
        return self.model_base.get_occlusion_culling_radius();
    }

    fn get_uniform(&self) -> &Uniform {
        return &self.model_base.get_uniform();
    }

    fn render_gbuffer(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        self.model_base.render_gbuffer(cmd, frame_number);
    }

    fn render_shadow(&self, _: &mut CmdBuffer, _: usize) {
        vxlogf!("Widget does not make shadow");
    }
}

impl DefaultModel for Base {
    fn default(eng: &Engine) -> Self {
        Base {
            model_base: ModelBase::default(eng),
            sensitive: false,
        }
    }
}

impl Widget for Base {}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Label {
    pub base: Base,
    pub background_color: [f32; 4],
    pub font: Arc<RwLock<Font>>,
    pub text: String,
    pub text_size: f32,
    pub text_color: [f32; 4],
    pub size: f32,
}

impl Label {
    pub fn set_text(&mut self, text: &str, engine: &Engine) {
        self.text = text.to_string();
        self.create_text_mesh(engine);
    }

    pub fn set_font_with_file_name(&mut self, name: &str, engine: &Engine) {
        self.font = vxresult!(engine.get_asset_manager().get_font_manager().write()).load_ttf(name);
        self.create_text_mesh(engine);
    }

    pub fn set_text_size(&mut self, size: f32, engine: &Engine) {
        self.text_size = size;
        self.create_text_mesh(engine);
    }

    pub fn set_size(&mut self, size: f32, engine: &Engine) {
        self.size = size;
        self.create_text_mesh(engine);
    }

    pub fn set_text_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32, engine: &Engine) {
        self.text_color = [red, green, blue, alpha];
        self.create_text_mesh(engine);
    }

    pub fn set_background_color(
        &mut self,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        engine: &Engine,
    ) {
        self.background_color = [red, green, blue, alpha];
        self.create_text_mesh(engine);
    }

    pub fn create_text_mesh(&mut self, engine: &Engine) {
        // todo margin
        // todo alignment
        // todo multiline support
        let mesh = {
            if self.text.len() < 1 {
                if self.get_meshes().len() < 1 {
                    return;
                }
                self.clear_meshes();
                return;
            }
            let scale = Scale::uniform(self.text_size);
            let font = vxresult!(self.font.read());
            let font = font.get_font();
            let v_metrics = font.v_metrics(scale);
            let point = point(0.0, 0.0 + v_metrics.ascent);
            let glyphs: Vec<_> = font.layout(&self.text, scale, point).collect();
            let mut max_x = 0;
            let mut max_y = 0;
            let mut min_x = 0;
            let mut min_y = 0;
            for glyph in &glyphs {
                if let Some(bb) = glyph.pixel_bounding_box() {
                    let max = &bb.max;
                    let min = &bb.min;
                    if min.x < min_x {
                        min_x = min.x;
                    }
                    if min.y < min_y {
                        min_y = min.y;
                    }
                    if max.x > max_x {
                        max_x = max.x;
                    }
                    if max.y > max_y {
                        max_y = max.y;
                    }
                }
            }
            vxlogi!("{}-{}-{}-{}", max_x, max_y, min_x, min_y);
            let imgw = max_x as i32 + 5;
            let imgh = max_y as i32 + 5;
            let w = self.size * (imgw as f32 / imgh as f32);
            let h = self.size;
            let bg = [
                (self.background_color[0] * 255.0) as u32,
                (self.background_color[1] * 255.0) as u32,
                (self.background_color[2] * 255.0) as u32,
                (self.background_color[3] * 255.0) as u32,
            ];
            let fc = [
                (self.text_color[0] * 255.0) as u32,
                (self.text_color[1] * 255.0) as u32,
                (self.text_color[2] * 255.0) as u32,
                (self.text_color[3] * 255.0) as u32,
            ];
            let pixels_count = (imgw * imgh) as usize;
            let bytes_count = pixels_count << 2;
            let mut img = vec![0u8; bytes_count];
            for i in 0..bytes_count {
                img[i] = bg[i & 3] as u8;
            }
            for glyph in &glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|x, y, v| {
                        let x = (x + bounding_box.min.x as u32) as usize;
                        let y = (y + bounding_box.min.y as u32) as usize;
                        let i = (y * imgw as usize + x) << 2;
                        let v = (v * 255.0) as u32;
                        let inv = 255 - v;
                        img[i] = ((bg[0] * inv + fc[0] * v) >> 8) as u8;
                        img[i + 1] = ((bg[1] * inv + fc[1] * v) >> 8) as u8;
                        img[i + 2] = ((bg[2] * inv + fc[2] * v) >> 8) as u8;
                        img[i + 3] = ((bg[3] * inv + fc[3] * v) >> 8) as u8;
                    });
                }
            }
            let vertices = [
                w, h, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                //-----------------------------------------------------------------------
                w, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
                //-----------------------------------------------------------------------
                0.0, h, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                //-----------------------------------------------------------------------
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ];
            let indices = [0u32, 2, 1, 1, 2, 3];
            let mut material = Material::default(engine);
            let asset_manager = engine.get_asset_manager();
            material.base_color = vxresult!(asset_manager.get_texture_manager().write())
                .create_2d_with_pixels(imgw as u32, imgh as u32, engine, &img);
            material.finalize_textures_change(engine);
            let radius = math::Vector2::new(w, h);
            let radius = math::dot(radius, radius).sqrt();
            let mesh = MeshBase::new_with_material(material, &vertices, &indices, radius, engine);
            let mesh: Arc<RwLock<Mesh>> = Arc::new(RwLock::new(mesh));
            vxresult!(asset_manager.get_mesh_manager().write()).add(&mesh);
            mesh
        };
        self.add_mesh(mesh);
    }
}

impl CoreObject for Label {
    fn get_id(&self) -> Id {
        self.base.get_id()
    }
}

impl Object for Label {
    fn get_name(&self) -> Option<String> {
        self.base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.base.set_name(name);
        vxunimplemented!(); //it must update corresponding manager
    }

    fn disable_rendering(&mut self) {
        self.base.disable_rendering();
    }

    fn enable_rendering(&mut self) {
        self.base.enable_rendering()
    }

    fn is_rendarable(&self) -> bool {
        return self.base.is_rendarable();
    }
}

impl Transferable for Label {
    fn set_orientation(&mut self, q: &math::Quaternion<Real>) {
        self.base.set_orientation(q);
    }

    fn set_location(&mut self, l: &math::Vector3<Real>) {
        self.base.set_location(l);
    }

    fn get_location(&self) -> math::Vector3<Real> {
        return self.base.get_location();
    }

    fn move_local_z(&mut self, d: Real) {
        self.base.move_local_z(d);
    }

    fn move_local_x(&mut self, d: Real) {
        self.base.move_local_x(d);
    }

    fn rotate_local_x(&mut self, d: Real) {
        self.base.rotate_local_x(d);
    }

    fn rotate_global_z(&mut self, d: Real) {
        self.base.rotate_global_z(d);
    }
}

impl Model for Label {
    fn update(&mut self, scene: &Scene, camera: &Camera, frame_number: usize) {
        self.base.update(scene, camera, frame_number);
    }

    fn get_meshes(&self) -> &BTreeMap<Id, Arc<RwLock<Mesh>>> {
        return self.base.get_meshes();
    }

    fn clear_meshes(&mut self) {
        self.base.clear_meshes();
    }

    fn add_mesh(&mut self, mesh: Arc<RwLock<Mesh>>) {
        self.base.add_mesh(mesh);
    }

    fn bring_all_child_models(&self) -> Vec<(Id, Arc<RwLock<Model>>)> {
        return self.base.bring_all_child_models();
    }

    fn has_shadow(&self) -> bool {
        return self.base.has_shadow();
    }

    fn get_occlusion_culling_radius(&self) -> Real {
        return self.base.get_occlusion_culling_radius();
    }

    fn get_uniform(&self) -> &Uniform {
        return self.base.get_uniform();
    }

    fn render_gbuffer(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        self.base.render_gbuffer(cmd, frame_number);
    }

    fn render_shadow(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        self.base.render_shadow(cmd, frame_number);
    }
}

impl DefaultModel for Label {
    fn default(eng: &Engine) -> Self {
        let font = vxresult!(eng.get_asset_manager().get_font_manager().read())
            .get_default()
            .clone();
        Label {
            base: Base::default(eng),
            text: String::new(),
            text_size: 1f32,
            text_color: [1f32; 4],
            background_color: [0f32; 4],
            font,
            size: 0.15f32,
        }
    }
}

impl Widget for Label {}
