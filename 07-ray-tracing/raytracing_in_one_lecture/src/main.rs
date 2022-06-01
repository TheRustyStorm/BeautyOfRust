extern crate png; 
extern crate cgmath;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use cgmath::{InnerSpace, Vector3};
use std::io::{BufRead, BufReader, Error, ErrorKind};


fn write_color_buffer_to_ppm_file(width: usize, height: usize, buf: Vec<u8>) -> Result<(), Error>{
    if buf.len() % 4 != 0{
        return Err(Error::new(ErrorKind::Other, "Number of colors not divisible by 4"));
    }
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width as u32, height as u32); 
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&buf).unwrap(); // Save
    Ok(())
}

fn main() {
    let width = 255;
    let height = 255;
    let mut buf: Vec<u8> = Vec::with_capacity(width*height*4);
    for y in 0..height{
        for x in 0..width{
            buf.push(x as u8);
            buf.push(y as u8);
            buf.push(0);
            buf.push(255);
        }
    }

    match write_color_buffer_to_ppm_file(width, height, buf){
        Ok(()) => {println!("Finished");},
        Err(e) => {panic!("{e}");}
    }
}
