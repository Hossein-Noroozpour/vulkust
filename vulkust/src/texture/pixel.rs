#[derive(Debug)]
pub enum Format {
    UNKNOWN,
    RGB8,
    RGBA8,
}

fn pixel_length(f: Format) -> u32 {
    match f {
        Format::UNKNOWN => logf!("UNKNOWN pixel format."),
        Format::RGB8 => 3,
        Format::RGBA8 => 4,
    }
}
