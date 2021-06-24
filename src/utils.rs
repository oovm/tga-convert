use crate::{Config, Error};
use glob::GlobResult;
// use png::{BitDepth, ColorType, OutputInfo};
use std::{fs, fs::File, io::Write};
use image::codecs::{png::PngDecoder,tga};
use image::ImageDecoder;

#[derive(Clone, Debug)]
pub struct PNG {
    pub path: Box<str>,
    pub size: u64,
    pub ratio: f32,
}

impl PNG {
    pub fn new(path: &str) -> Self {
        Self { path: Box::from(path), size: 0, ratio: -1.0 }
    }

    pub fn size_ratio(&mut self) -> Result<(), Error> {
        let (info, _) = png::Decoder::new(File::open(&*self.path)?).read_info()?;
        self.size = fs::metadata(&*self.path)?.len();
        self.ratio = *&self.size as f32 / estimate_size(&info);
        return Ok(());
    }
}

pub fn write_to_file(path: &str, bytes: &[u8]) -> Result<(), Error> {
    println!("Generating {}", path);
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    Ok(())
}

pub fn check_file(entry: GlobResult, cfg: &Config) -> Result<String, Error> {
    let mut image = Vec::new();
    let path = &entry?.to_path_buf();
    PngDecoder::new(File::open(path)?)?.read_image(&mut image)?;



}
