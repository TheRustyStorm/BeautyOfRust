extern crate png; 
extern crate cgmath;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use cgmath::{InnerSpace, Vector3};
use std::io::{BufRead, BufReader, Error, ErrorKind};

struct Ray{
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray{
    fn get_position_on_ray(&self, t: f64) -> Vector3<f64>{
        self.origin + t * self.direction
    }
}

struct Sphere{
    origin: Vector3<f64>,
    radius: f64,
    color: Vector3<f64>,
}


fn write_color_buffer_to_png_file(width: usize, height: usize, buf: Vec<u8>) -> Result<(), Error>{
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

fn ray_color(r: &Ray) -> Vector3<f64>{
    let sphere = Sphere{origin: Vector3::new(0.0,0.0,6.5), radius: 1.0, color: Vector3::new(1.0,0.7,0.5)};
    let t =  hit_sphere(&sphere, &r);
    if t > 0.0{
        let n = r.get_position_on_ray(t) - Vector3::new(0.0,0.0,-1.0);
        return 0.5*Vector3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0-t)*Vector3::new(1.0, 1.0, 1.0) + t*Vector3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(sphere: &Sphere, ray: &Ray) -> f64{
    let oc = ray.origin - sphere.origin;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - sphere.radius * sphere.radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        return -1.0;
    } else{
        return (-b - discriminant.sqrt()) / (2.0 * a)
    }
}   


fn main() {

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u64 = 1000;
    let image_height : u64  = (image_width as f64 / aspect_ratio) as u64;

    let viewport_height = 1.0;
    let viewport_width = aspect_ratio * viewport_height;
    
    // what does this do? set to 1?
    let focal_length = 1.0;

    let origin:Vector3<f64> = Vector3::new(0.0,0.0,0.0);

    // set to 1 dummy?
    let horizontal: Vector3<f64> = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical: Vector3<f64> = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 + Vector3::new(0.0, 0.0 ,focal_length);

    let mut buf: Vec<u8> = Vec::with_capacity(image_width as usize *image_height as usize*4 );
    for y in (0..image_height).rev(){
        for x in 0..image_width{
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;
            let ray = Ray{origin: origin, direction: lower_left_corner + u*horizontal + v*vertical - origin};
            let color = ray_color(&ray);
            buf.push((color.x*255.0) as u8);
            buf.push((color.y*255.0) as u8);
            buf.push((color.z*255.0) as u8);
            buf.push(255);
        }
    }

    match write_color_buffer_to_png_file(image_width as usize, image_height as usize, buf){
        Ok(()) => {println!("Finished");},
        Err(e) => {panic!("{e}");}
    }
}
