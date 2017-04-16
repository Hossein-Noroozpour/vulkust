use std::collections::{
    HashMap,
    HashSet,
};
use std::rc::Weak;

use ::texture::{
    Texture,
    TextureType,
};
use ::texture::texture_2d::Texture2D;
use ::texture::cube_map::CubeMap;
use ::io::file::Stream;

pub struct TexturesManager {
    texture_entries: Vec<TextureEntry>,
    name_index: HashMap<String, usize>,
    teture_2ds: Vec<Weak<Texture2D>>,
    name_index_2ds: HashMap<String, usize>,
    cubemaps: Vec<Weak<CubeMap>>,
    name_index_cubemaps: HashMap<String, usize>,
}

struct TextureEntry {
    index: u16,
    offset: u64,
    texture_type: TextureType,
}

impl TexturesManager {
    pub fn new() -> TexturesManager {
        TexturesManager {
            texture_entries: Vec::new(),
            name_index: HashMap::new(),
            teture_2ds: Vec::new(),
            name_index_2ds: HashMap::new(),
            cubemaps: Vec::new(),
            name_index_cubemaps: HashMap::new(),
        }
    }

    pub fn read_table(&mut self, s: &mut Stream) {
        let texture_count = s.read(&0u16);
        println!("Texture count: {:?}", texture_count);
        for _ in 0..texture_count {
            let names_count = s.read(&0u8);
            println!("Texture name count: {:?}", names_count);
            let mut names = HashSet::new();
            for _ in 0..names_count {
                let name = s.read_string();
                println!("Texture name: {:?}", name);
                names.insert(name);
            }
            let index = s.read(&0u16);
            let texture_type: TextureType;
            match s.read(&0u8) {
                1 => {
                    texture_type = TextureType::Texture2D;
                }
                2 => {
                    texture_type = TextureType::CubeMap;
                }
                _ => {
                    logf!("Unknown texture type.");
                }
            }
            let offset = s.read(&0u32) as u64;
            let texture_entry = TextureEntry {
                index: index,
                offset: offset,
                texture_type: texture_type,
            };
            let entry_index = self.texture_entries.len();
            for name in names {
                self.name_index.insert(name, entry_index);
            }
            self.texture_entries.push(texture_entry);
        }
    }
}
