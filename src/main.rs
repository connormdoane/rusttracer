use std::fs::File;
use std::io::prelude::*;
pub mod geometry;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const FOV: f64 = 0.5;

fn vec3f(x0: f64, y0: f64, z0: f64) -> geometry::Vec3f {
    geometry::Vec3f{x: x0, y: y0, z: z0}
}

struct Sphere {
    center: geometry::Vec3f,
    radius: f64,
    material: Material,
}

fn get_sphere(v: geometry::Vec3f, r: f64, mat: Material) -> Sphere {
    Sphere{center: v, radius: r, material: mat}
}

#[derive(Copy, Clone)]
struct Material {
    diffuse_color: geometry::Vec3f,
}

fn red_rubber() -> Material {
    Material{diffuse_color: vec3f(0.3, 0.1, 0.1)}
}

fn ivory() -> Material {
    Material{diffuse_color: vec3f(0.4, 0.4, 0.4)}
}

struct Light {
    position: geometry::Vec3f,
    intensity: f64,
}

fn get_light(v: geometry::Vec3f, i: f64) -> Light {
    Light{position: v, intensity: i}
}

impl Sphere {
    fn ray_intersect(&self, orig: geometry::Vec3f, dir: geometry::Vec3f, t0: &mut f64) -> bool {
	let l: geometry::Vec3f = self.center - orig;
	let tca: f64 = l.dot_product(dir);
	let d2: f64 = l.dot_product(l) - tca*tca;
	if d2 > self.radius * self.radius { return false }
	let thc: f64 = f64::sqrt((self.radius * self.radius) - d2);
	*t0 = tca - thc;
	let t1: f64 = tca + thc;
	if t0 < &mut 0. { *t0 = t1 };
	if t0 < &mut 0. { return false };
	true
    }
}

fn scene_intersect(orig: geometry::Vec3f, dir: geometry::Vec3f, spheres: &mut Vec<Sphere>, hit: &mut geometry::Vec3f, n: &mut geometry::Vec3f, material: &mut Material) -> bool {
    let mut spheres_dist: f64 = f64::MAX;
    for i in 0..spheres.len() {
	let mut dist_i: f64 = 0.;
	if spheres[i].ray_intersect(orig, dir, &mut dist_i) && dist_i < spheres_dist {
	    spheres_dist = dist_i;
	    *hit = vec3f(orig.x + (dir.x * dist_i), orig.y + (dir.y * dist_i), orig.z + (dir.z * dist_i));
	    *n = (*hit - spheres[i].center).normalize();
	    *material = spheres[i].material;
	}
    }
    spheres_dist < 1000.
}

fn cast_ray(orig: geometry::Vec3f, dir: geometry::Vec3f, spheres: &mut Vec<Sphere>, lights: &mut Vec<Light>) -> geometry::Vec3f {
    let mut point: geometry::Vec3f = vec3f(0., 0., 0.);
    let mut n: geometry::Vec3f = vec3f(0., 0., 0.);
    let mut material: Material = Material{diffuse_color: vec3f(0., 0., 0.)};

    if !scene_intersect(orig, dir, spheres, &mut point, &mut n, &mut material) {
	return vec3f(0.2, 0.7, 0.8); // background color
    }
    let mut diffuse_light_intensity: f64 = 0.;
    for i in 0..lights.len() {
	let light_dir: geometry::Vec3f = (lights[i].position - point).normalize();
	diffuse_light_intensity += lights[i].intensity * light_dir.dot_product(n); // potentially max with 0.
    }
    vec3f(material.diffuse_color.x * diffuse_light_intensity, material.diffuse_color.y * diffuse_light_intensity, material.diffuse_color.z * diffuse_light_intensity)
}

fn save_to_file(file_path: &str, frame_buffer: Vec<geometry::Vec3f>) -> std::io::Result<()> {
    let mut file: File = File::create(file_path)?;
    file.write_all(b"P3\n1024 768\n255\n")?;
    for i in 0usize..(HEIGHT*WIDTH) as usize {
	for j in 0usize..3usize {
	    if frame_buffer[i][j] < 0. {
		write!(file, "0")?;
	    } else if frame_buffer[i][j] > 1. {
		write!(file, "255")?;
	    } else {
		write!(file, "{}", ((255 as f64 * frame_buffer[i][j]) as u16).to_string())?;
	    }
	    write!(file, " ")?;
	}
	write!(file, "\n")?;
    }
    Ok(())
}

fn _gradient(mut frame_buffer: Vec<geometry::Vec3f>) -> Vec<geometry::Vec3f> {
    for j in 0..HEIGHT {
	for i in 0..WIDTH {
	    frame_buffer.push(geometry::Vec3f{x: j as f64/HEIGHT as f64, y: i as f64/WIDTH as f64, z: 0.});
	}
    }
    frame_buffer
}

fn render(spheres: &mut Vec<Sphere>, lights: &mut Vec<Light>) {
    let file_path = "output.ppm";
    let mut frame_buffer: Vec<geometry::Vec3f> = Vec::with_capacity(WIDTH as usize*HEIGHT as usize);
    for _i in 0..WIDTH*HEIGHT {
	frame_buffer.push(geometry::Vec3f{x: 0., y: 0., z: 0.});
    }
    for j in 0..HEIGHT {
	for i in 0..WIDTH {
	    let xp: f64 = (2.*(i as f64+0.5) as f64/WIDTH as f64-1.)*f64::tan(FOV)*WIDTH as f64/HEIGHT as f64;
	    let yp: f64 = -(2.*(j as f64+0.5) as f64/HEIGHT as f64-1.)*f64::tan(FOV);
	    let dir: geometry::Vec3f = geometry::Vec3f{x: xp, y: yp, z: -1.}.normalize();
	    frame_buffer[(i+j*WIDTH) as usize] = cast_ray(geometry::Vec3f{x: 0., y: 0., z: 0.}, dir, spheres, lights);
	}
    }
    let _file = save_to_file(file_path, frame_buffer);
    ()
}

fn main() -> std::io::Result<()> {
//    let test1 = geometry::Vec3i{x: 5, y: 5, z: 5};
//    let test2 = geometry::Vec3i{x: 2, y: 2, z: 2};
//    assert!(test1.clone()*test2.clone() == geometry::Vec3i{x: 10, y: 10, z: 10});
//    assert!(test1.clone()+test2.clone() == geometry::Vec3i{x: 7, y: 7, z: 7});
//    assert!(test1.clone()-test2.clone() == geometry::Vec3i{x: 3, y: 3, z: 3});
    let mut spheres: Vec<Sphere> = vec![];
    spheres.push(get_sphere(vec3f(-3., 0., -16.), 2., ivory()));
    spheres.push(get_sphere(vec3f(-1.0, -1.5, -12.), 2., red_rubber()));
    spheres.push(get_sphere(vec3f(1.5, -0.5, -18.), 3., red_rubber()));
    spheres.push(get_sphere(vec3f(7., 5., -18.), 4., ivory()));

    let mut lights: Vec<Light> = vec![];
    lights.push(get_light(vec3f(-20., 20., 20.), 1.5));
    
    render(&mut spheres, &mut lights);
    Ok(())
}
