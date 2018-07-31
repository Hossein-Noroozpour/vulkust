use super::super::core::object::Object as CoreObject;
use super::super::core::types::Id;
use super::engine::Engine;
use super::font::Font;
use super::mesh::{Base as MeshBase, Mesh};
use super::model::{Base as ModelBase, Model};
use super::object::Object;
use super::scene::Uniform as SceneUniform;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use rusttype::{point, Scale};

pub trait Widget: Model {}

#[cfg_attr(debug_assertions, derive(Debug))]
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

    fn render(&self, engine: &Engine) {
        self.model_base.render(engine);
    }

    fn disable_rendering(&mut self) {
        self.model_base.disable_rendering();
    }

    fn enable_rendering(&mut self) {
        self.model_base.enable_rendering()
    }

    fn update(&mut self) {
        self.model_base.update();
    }
}

impl Model for Base {}

impl Widget for Base {}

#[cfg_attr(debug_assertions, derive(Debug))]
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
    pub fn set_text(&mut self, text: &str, engine: &Arc<RwLock<Engine>>) {
        self.text = text.to_string();
        self.create_text_mesh(engine);
    }

    pub fn set_font_with_file_name(&mut self, name: &str, engine: &Arc<RwLock<Engine>>) {
        {
            let engine = vxresult!(engine.read());
            let scene_manager = vxresult!(engine.scene_manager.read());
            let mut font_manager = vxresult!(scene_manager.font_manager.write());
            self.font = font_manager.load_ttf(name);
        }
        self.create_text_mesh(engine);
    }

    pub fn set_text_size(&mut self, size: f32, engine: &Arc<RwLock<Engine>>) {
        self.text_size = size;
        self.create_text_mesh(engine);
    }

    pub fn set_text_color(
        &mut self,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        engine: &Arc<RwLock<Engine>>,
    ) {
        self.text_color = [red, green, blue, alpha];
        self.create_text_mesh(engine);
    }

    pub fn set_background_color(
        &mut self,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        engine: &Arc<RwLock<Engine>>,
    ) {
        self.background_color = [red, green, blue, alpha];
        self.create_text_mesh(engine);
    }

    pub fn create_text_mesh(&mut self, engine: &Arc<RwLock<Engine>>) {
        // todo margin
        // todo alignment
        // todo multiline support
        if self.text.len() < 1 {
            if self.base.model_base.meshes.len() < 1 {
                return;
            }
            self.base.model_base.meshes = BTreeMap::new();
            return;
        }
        let scale = Scale::uniform(self.text_size);
        let font = vxresult!(self.font.read());
        let font = font.get_font();
        let v_metrics = font.v_metrics(scale);
        let point = point(0.0, 0.0 + v_metrics.ascent);
        let glyphs: Vec<_> = font.layout(&self.text, scale, point).collect();
        let glyphs_len = glyphs.len();
        let imgbb = vxunwrap!(glyphs[glyphs_len - 1].pixel_bounding_box()).max;
        let imgw = imgbb.x as i32 + 5;
        let imgh = imgbb.y as i32 + 5;
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
                    img[i + 3] = 0; //((bg[3] * inv + fc[3] * v) >> 8) as u8;
                });
            }
        }
        let vertices = [
            w, h, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, w, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 1.0, 1.0, 0.0, h, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        let indices = [0u32, 2, 1, 1, 2, 3];
        let eng = vxresult!(engine.read());
        let scene_manager = vxresult!(eng.scene_manager.read());
        let mut texture_manager = vxresult!(scene_manager.texture_manager.write());
        let texture =
            texture_manager.create_2d_with_pixels(imgw as u32, imgh as u32, &engine, &img);
        // todo
        // let mesh = MeshBase::new_with_material(texture, &vertices, &indices, &eng);
        // let mesh_id = mesh.get_id();
        // let mesh = Arc::new(RwLock::new(mesh));
        // self.base.model_base.meshes.insert(mesh_id, mesh);
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

    fn render(&self, engine: &Engine) {
        self.base.render(engine);
    }

    fn disable_rendering(&mut self) {
        self.base.disable_rendering();
    }

    fn enable_rendering(&mut self) {
        self.base.enable_rendering()
    }

    fn update(&mut self) {
        self.base.update();
    }
}

impl Model for Label {}

// impl DefaultMesh for Label {
//     fn default(engine: &Arc<RwLock<Engine>>) -> Self {
//         let eng = vxresult!(engine.read());
//         let scene_manager = vxresult!(eng.scene_manager.read());
//         let font_manager = vxresult!(scene_manager.font_manager.read());
//         let font = font_manager.default.clone();
//         Label {
//             base: Base::default(engine),
//             text: String::new(),
//             text_size: 1f32,
//             text_color: [1f32; 4],
//             background_color: [0f32; 4],
//             font,
//             size: 0.15f32,
//         }
//     }
// }

impl Widget for Label {}
