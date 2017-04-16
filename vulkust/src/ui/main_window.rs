extern crate gtk;
extern crate image;
extern crate gdk_pixbuf;

use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::fs::File;

use self::gtk::prelude::*;

use self::gdk_pixbuf::{
    Pixbuf,
    Colorspace
};

use self::image::{
    ImageLuma8,
    ImageLumaA8,
    ImageRgb8,
    ImageRgba8
};

pub struct MainWindow {
    win : gtk::Window,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_title("Dust, Gearonix software");
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(350, 70);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        let f = File::open("/home/thany/Pictures/1.png").unwrap();
        let mut reader = BufReader::new(f);
        let pngsize = reader.seek(SeekFrom::End(0)).unwrap() as usize;
        reader.seek(SeekFrom::Start(0)).expect("Unable to seek.");
        let mut data = Box::new(Vec::<u8>::new)();
        data.resize(pngsize, 0u8);
        let len = reader.read(&mut data).unwrap();
        println!("Image bytes count is {} and read bytes is {}", pngsize, len);
        let imghasalpha: bool;
        let imgbps: i32;
        let imgw: i32;
        let imgh: i32;
        let imgs: i32;
        match image::load_from_memory(&data) {
            Ok(img) => {
                data = img.raw_pixels();
                match img {
                    ImageLuma8(_) => logf!("Not supported yet."),
                    ImageLumaA8(_) => logf!("Not supported yet."),
                    ImageRgb8(dimg) => {
                        imgw = dimg.width() as i32;
                        imgh = dimg.height() as i32;
                        imgs = data.len() as i32 / imgh;
                        imgbps = 8i32;
                        imghasalpha = false;
                    }
                    ImageRgba8(dimg) => {
                        imgw = dimg.width() as i32;
                        imgh = dimg.height() as i32;
                        imgs = data.len() as i32 / imgh;
                        imgbps = 8i32;
                        imghasalpha = true;
                    }
                }
            }
            Err(err) => {
                logf!("Error {:?} in image library", err);
            }
        }
        let imgpixbuf = Box::new(Pixbuf::new_from_vec)(data, 0 as Colorspace, imghasalpha, imgbps, imgw, imgh, imgs);
        let imgwidget = Box::new(gtk::Image::new_from_pixbuf)(Some(&imgpixbuf));
        window.add(&imgwidget);
        MainWindow { win: window }
    }

    pub fn show(&self) {
        self.win.show_all();
    }
}
