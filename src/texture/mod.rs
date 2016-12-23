extern crate image;

pub mod cube_map;
pub mod pixel;
pub mod texture_2d;
pub mod textures_manager;

use std::fs::File;
use std::io::{
    BufReader,
    SeekFrom,
    Seek,
    Read,
};
use std::rc::Rc;

use self::image::{
    ImageLuma8,
    ImageLumaA8,
    ImageRgb8,
    ImageRgba8
};

enum TextureType {
    Texture2D,
    CubeMap,
}

#[derive(Debug)]
pub struct Texture {
    width: u32,
    height: u32,
    format: pixel::Format,
    bitmap: Vec<u8>,
}

impl Texture {
    fn new_from_file(filename: &str) -> Rc<Texture> {
        println!("Trying to load image {}", filename);
        let f = File::open(filename).unwrap();
        let mut reader = BufReader::new(f);
        let pngsize = reader.seek(SeekFrom::End(0)).unwrap() as usize;
        reader.seek(SeekFrom::Start(0)).expect("Unable to seek.");
        let mut data = Box::new(Vec::<u8>::new)();
        data.resize(pngsize, 0u8);
        let len = reader.read(&mut data).unwrap();
        println!("Image bytes count is {} and read bytes is {}", pngsize, len);
        let imgf: pixel::Format;
        let imgw: u32;
        let imgh: u32;
        match image::load_from_memory(&data) {
            Ok(img) => {
                data = img.raw_pixels();
                match img {
                    ImageLuma8(_) => panic!("Not supported yet."),
                    ImageLumaA8(_) => panic!("Not supported yet."),
                    ImageRgb8(dimg) => {
                        imgw = dimg.width() as u32;
                        imgh = dimg.height() as u32;
                        imgf = pixel::Format::RGB8;
                    }
                    ImageRgba8(dimg) => {
                        imgw = dimg.width() as u32;
                        imgh = dimg.height() as u32;
                        imgf = pixel::Format::RGBA8;
                    }
                }
            }
            Err(err) => {
                println!("{:?}", err);
                panic!("Error in image library");
            }
        }
        Rc::new(Texture {
            width: imgw,
            height: imgh,
            format: imgf,
            bitmap: data,
        })
    }

    fn new() -> Texture {
        Texture {
            width: 0u32,
            height: 0u32,
            format: pixel::Format::UNKNOWN,
            bitmap: Vec::new(),
        }
    }

    // fn get_pixel_starting_index(u: f64, v: f64) -> u32 {
    //     /// TODO
    // }
}
