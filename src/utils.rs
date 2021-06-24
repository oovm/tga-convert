// use png::{BitDepth, ColorType, OutputInfo};
// use std::{fs, fs::File, io::Write};

use glob::GlobResult;
use image::{ ImageFormat};
// use image::codecs::{png::PngDecoder, tga};
use image::io::Reader;

use crate::{Config, Error};

pub fn export_tga(entry: GlobResult, _cfg: &Config) -> Result<(), Error> {
    let path = &entry?.to_path_buf();
    let name = path.file_name()?.to_str()?;
    let image = Reader::open(path)?.decode()?;
    let out = path.parent()?.join(format!("{}.tga", name));
    println!("{:?}",out);
    image.save_with_format(out, ImageFormat::Tga)?;
    Ok(())
}
