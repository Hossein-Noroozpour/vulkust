extern crate image;
extern crate rusttype;
#[macro_use]
extern crate vulkust;
extern crate clap;

use clap::{App, Arg};
use image::{DynamicImage, Rgba};
use rusttype::{point, Font, Scale};

use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn round_power_2(v: i32) -> i32 {
    let mut x = 64;
    while x < v {
        x <<= 1;
    }
    return x;
}

fn main() {
    let argmatches = App::new("TTF-Baker")
        .version("0.1")
        .author("Hossein Noroozpour <hossein.noroozpour@gmail.com>")
        .about("Does ttf baking that is needed for Vulkust game engine.")
        .arg(
            Arg::with_name("font")
                .short("f")
                .long("font")
                .help("Sets font file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .help("Sets font sclaing")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("aspect")
                .short("a")
                .long("aspect")
                .help("Sets image aspect")
                .takes_value(true)
                .required(false),
        )
        .get_matches();
    let imga = vxresult!(i32::from_str(
        argmatches.value_of("aspect").unwrap_or("1024")
    ));
    let font_file_name = vxunwrap_o!(argmatches.value_of("font"));
    let mut font_file = vxresult!(File::open(font_file_name));
    let mut font_data = Vec::new();
    vxresult!(font_file.read_to_end(&mut font_data));
    let font = vxresult!(Font::from_bytes(&font_data));
    let font_scale = vxresult!(f32::from_str(
        argmatches.value_of("scale").unwrap_or("74.0")
    ));
    let paddingf = (font_scale / 10.0).ceil();
    let paddingi = paddingf as i32;
    let scale = Scale::uniform(font_scale);
    let mut text = String::new();
    let starting_char = 33i32;
    let ending_char = 127i32;
    let chars_number = ending_char - starting_char;
    for i in starting_char..ending_char {
        text += &(i as u8 as char).to_string();
    }
    let interleaved_space = chars_number * paddingi;
    let colour = (255, 255, 255); // todo change it to black and white
    let v_metrics = font.v_metrics(scale);
    vxlogi!("{:?}", &v_metrics);
    let charh = v_metrics.ascent - v_metrics.descent;
    let line_base_gap = (charh + v_metrics.line_gap) / font_scale;
    vxlogi!(
        "Line base gap: {}, Interleaved space: {}",
        line_base_gap,
        interleaved_space
    );
    let line_pos = paddingf + v_metrics.ascent;
    let glyphs: Vec<_> = font
        .layout(&text, scale, point(paddingf, paddingf + v_metrics.ascent))
        .collect();
    let imgw = round_power_2(imga);
    let imghinc = (paddingf + charh.ceil()) as i32;
    let mut imgh = imghinc;
    let mut incx = 0i32;
    let mut row_count = 1;
    for glyph in &glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            if bounding_box.max.x + incx + paddingi > imgw as i32 {
                incx = paddingi as i32 - bounding_box.min.x;
                imgh += imghinc;
                row_count += 1;
            }
            incx += paddingi as i32;
        }
    }
    let imgh = round_power_2(imgh);
    let mut incy = 0i32;
    let mut incx = 0i32;
    let mut image = DynamicImage::new_rgba8(imgw as u32, imgh as u32).to_rgba();
    let mut metas = Vec::new();
    let mut row = Vec::new();
    let mut upline = paddingf;
    let mut loline = upline + charh.ceil();
    for glyph in &glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            if bounding_box.max.x + incx + paddingi > imgw as i32 {
                incx = paddingi as i32 - bounding_box.min.x;
                incy += imghinc;
                metas.push((row.clone(), upline, loline));
                upline = loline + paddingf;
                loline = upline + charh.ceil();
                row.clear();
            }
            row.push((bounding_box.min.x, bounding_box.max.x));
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    (x as i32 + bounding_box.min.x + incx) as u32,
                    (y as i32 + bounding_box.min.y + incy) as u32,
                    Rgba {
                        data: [colour.0, colour.1, colour.2, (v * 255.0) as u8],
                    },
                )
            });
            incx += paddingi as i32;
        }
    }
    metas.push((row.clone(), upline, loline));

    for meta in &metas {
        for x in 0..imgw {
            image.put_pixel(
                x as u32,
                meta.1 as u32,
                Rgba {
                    data: [255, 0, 0, 255],
                },
            )
        }
        for x in 0..imgw {
            image.put_pixel(
                x as u32,
                meta.2 as u32,
                Rgba {
                    data: [0, 255, 0, 255],
                },
            )
        }
    }

    // Save the image to a png file
    image.save("image_example.png").unwrap();
    println!("Generated: image_example.png");
}
