#![allow(unused_assignments)]
use num;
use std::fs::File;
use std::io::prelude::*;
pub mod geometry;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const ENVMAP_WIDTH: u32 = 4096;
const ENVMAP_HEIGHT: u32 = 2048;
const FOV: f64 = 0.5;

fn vec3f(x0: f64, y0: f64, z0: f64) -> geometry::Vec3f {
    geometry::Vec3f{x: x0, y: y0, z: z0}
}

fn _vec2f(x0: f64, y0: f64) -> geometry::Vec2f {
    geometry::Vec2f{x: x0, y: y0}
}

fn vec4f(x0: f64, y0: f64, z0: f64, a0: f64) -> geometry::Vec4f {
    geometry::Vec4f{x: x0, y: y0, z: z0, a: a0}
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
    albedo: geometry::Vec4f,
    specular_exponent: f64,
    refractive_index: f64,
}

fn ivory() -> Material {
    Material{diffuse_color: vec3f(0.4, 0.4, 0.3), albedo: vec4f(0.6, 0.3, 0.1, 0.0), specular_exponent: 50., refractive_index: 1.0}
}

fn red_rubber() -> Material {
    Material{diffuse_color: vec3f(0.3, 0.1, 0.1), albedo: vec4f(0.9, 0.1, 0.0, 0.0), specular_exponent: 10., refractive_index: 1.0}
}

fn mirror() -> Material {
    Material{diffuse_color: vec3f(1.0, 1.0, 1.0), albedo: vec4f(0.0, 10.0, 0.8, 0.0), specular_exponent: 1425., refractive_index: 1.0}
}

fn glass() -> Material {
    Material{diffuse_color: vec3f(0.6, 0.7, 0.8), albedo: vec4f(0.0, 0.5, 0.1, 0.8), specular_exponent: 125., refractive_index: 1.5}
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

//fn reflect(i: geometry::Vec3f, n: geometry::Vec3f) -> geometry::Vec3f {
//    i - (vec3f(n.x*(2. * i.dot_product(n)), n.y*(2. * i.dot_product(n)), n.z*(2. * i.dot_product(n))))
//}

fn reflect(i: geometry::Vec3f, n: geometry::Vec3f) -> geometry::Vec3f {
    let dot: f64 = i.dot_product(n);
    i - vec3f(n.x*2.*dot, n.y*2.*dot, n.z*2.*dot)
}

fn refract(i: geometry:: Vec3f, n: geometry::Vec3f, refractive_index: f64) -> geometry::Vec3f {
    let mut cosi: f64 = -f64::max(-1., f64::min(1., i.dot_product(n)));
    let mut etai: f64 = 1.;
    let mut etat: f64 = refractive_index;
    let mut n0: geometry::Vec3f = n;
    if cosi < 0. {
	cosi = -cosi;
	std::mem::swap(&mut etai, &mut etat);
	n0 = vec3f(-n.x, -n.y, -n.z);
    }
    let eta: f64 = etai / etat;
    let k: f64 = 1. - eta * eta * (1.-cosi*cosi);
    let reflection: f64 = eta*cosi-(k).sqrt();
    if k < 0. {vec3f(0., 0., 0.)} else {vec3f(i.x*eta, i.y*eta, i.z*eta) + vec3f(n0.x*reflection, n0.y*reflection, n0.z*reflection)}
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
    let mut checkerboard_dist: f64 = f64::MAX;
    if f64::abs(dir.y)>0.001 {
	let d: f64 = -(orig.y + 4.)/dir.y;
	let pt0: geometry::Vec3f = vec3f(dir.x*d, dir.y*d, dir.z*d);
	let pt: geometry::Vec3f = orig + pt0;
	if d > 0. && f64::abs(pt.x) < 10. && pt.z < -10. && pt.z > -30. && d < spheres_dist {
	    checkerboard_dist = d;
	    *hit = pt;
	    *n = vec3f(0., 1., 0.);
	    material.diffuse_color = if ((0.5*hit.x+1000.) as i64 + (0.5*hit.z) as i64) & 1 == 1 {vec3f(0.3,0.3,0.3)} else {vec3f(0.3, 0.2, 0.1)};
	    material.albedo = vec4f(1., 0., 0., 0.);
	}
    }
    
    f64::min(spheres_dist, checkerboard_dist) < 1000.
}

fn cast_ray(orig: geometry::Vec3f, dir: geometry::Vec3f, spheres: &mut Vec<Sphere>, lights: &mut Vec<Light>, depth: usize, envmap: &Vec<geometry::Vec3<f64>>) -> geometry::Vec3f {
    let mut point: geometry::Vec3f = vec3f(0., 0., 0.);
    let mut n: geometry::Vec3f = vec3f(0., 0., 0.);
    let mut material: Material = Material{diffuse_color: vec3f(0., 0., 0.), albedo: vec4f(0., 0., 0., 0.), specular_exponent: 0., refractive_index: 0.};

    if depth > 4 || !scene_intersect(orig, dir, spheres, &mut point, &mut n, &mut material) {
	//return vec3f(0.2, 0.7, 0.8); // background color
	let a: u32 = num::clamp((((dir.z.atan2(dir.x) / (2.*std::f64::consts::PI) + 0.5) * ENVMAP_WIDTH as f64)) as u32, 0, ENVMAP_WIDTH - 1);
	let b: u32 = num::clamp((dir.y.acos() / std::f64::consts::PI*ENVMAP_HEIGHT as f64) as u32, 0, ENVMAP_HEIGHT - 1);
	return envmap[(a + b * ENVMAP_WIDTH) as usize]
    }
    let reflect_dir: geometry::Vec3f = reflect(dir, n);
    let refract_dir: geometry::Vec3f = refract(dir, n, material.refractive_index).normalize();
    let mirror_n: geometry::Vec3f = vec3f(n.x*0.001, n.y*0.001, n.z*0.001);
    let reflect_orig: geometry::Vec3f = if reflect_dir.dot_product(n) < 0. {point - mirror_n} else {point + mirror_n};
    let refract_orig: geometry::Vec3f = if refract_dir.dot_product(n) < 0. {point - mirror_n} else {point + mirror_n};
    let reflect_color: geometry::Vec3f = cast_ray(reflect_orig, reflect_dir, spheres, lights, depth+1, &envmap);
    let refract_color: geometry::Vec3f = cast_ray(refract_orig, refract_dir, spheres, lights, depth+1, &envmap);
    let mut diffuse_light_intensity: f64 = 0.;
    let mut specular_light_intensity: f64 = 0.;
    for i in 0..lights.len() {
	let light_dir: geometry::Vec3f = (lights[i].position - point).normalize();
	let light_distance: f64 = (lights[i].position - point).norm();
	let shadow_ns: geometry::Vec3f = vec3f(n.x*0.001, n.y*0.001, n.z*0.001);
	let shadow_orig: geometry::Vec3f = if light_dir.dot_product(n) < 0. { point - shadow_ns } else { point + shadow_ns };
	let mut shadow_pt: geometry::Vec3f = vec3f(0., 0., 0.);
	let mut shadow_n: geometry::Vec3f = vec3f(0., 0., 0.);
	let mut shadow_material: Material = ivory();
	if scene_intersect(shadow_orig, light_dir, spheres, &mut shadow_pt, &mut shadow_n, &mut shadow_material) && (shadow_pt-shadow_orig).norm() < light_distance {continue}
	diffuse_light_intensity += lights[i].intensity * f64::max(0., light_dir.dot_product(n)); // potentially max with 0.
	specular_light_intensity += f64::max(0., reflect(light_dir, n).dot_product(dir)).powf(material.specular_exponent) * lights[i].intensity
    }
    let diffuse: f64 = diffuse_light_intensity * material.albedo[0];
    let specular: f64 = specular_light_intensity * material.albedo[1];
    vec3f(material.diffuse_color.x * diffuse, material.diffuse_color.y * diffuse, material.diffuse_color.z * diffuse) + vec3f(specular, specular, specular) + vec3f(reflect_color.x*material.albedo[2], reflect_color.y*material.albedo[2], reflect_color.z*material.albedo[2]) + vec3f(refract_color.x*material.albedo[3], refract_color.y*material.albedo[3], refract_color.z*material.albedo[3])
}

fn save_to_file(file_path: &str, frame_buffer: Vec<geometry::Vec3f>) -> std::io::Result<()> {
    let mut file: File = File::create(file_path)?;
    file.write_all(b"P3\n1920 1080\n65535\n")?;
    for i in 0usize..(1920*1080) as usize {
	let mut c: geometry::Vec3f = frame_buffer[i];
	let max = f64::max(c[0], f64::max(c[1], c[2]));
	if max>1. {c = vec3f(c.x * (1./max), c.y * (1./max), c.z * (1./max))}
	for j in 0usize..3usize {
	    if frame_buffer[i][j] < 0. {
		write!(file, "0")?;
	    } else if frame_buffer[i][j] > 1. {
		write!(file, "65535")?;
	    } else {
		write!(file, "{}", ((65535 as f64 * frame_buffer[i][j]) as u16).to_string())?;
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

fn render(spheres: &mut Vec<Sphere>, lights: &mut Vec<Light>, envmap: &Vec<geometry::Vec3f>) {
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
	    frame_buffer[(i+j*WIDTH) as usize] = cast_ray(geometry::Vec3f{x: 0., y: 0., z: 0.}, dir, spheres, lights, 0, &envmap);
	}
    }
    let _file = save_to_file(file_path, frame_buffer);
    ()
}

fn build_envmap(file_path: &str) -> Vec<geometry::Vec3f> {
    let mut op: Vec<geometry::Vec3f> = vec![];
    let file: String = std::fs::read_to_string(file_path).unwrap();
    let mut r: f64 = 0.;
    let mut g: f64 = 0.;
    let lines: Vec<_> = file.split('\n').collect();
//    for line in lines {
    for i in 0..lines.len()-1 {
	if i >= 3 {
	    if i % 3 == 0 {
		r = lines[i].parse::<f64>().unwrap()/65535.;
	    }
	    if i % 3 == 1 {
		g = lines[i].parse::<f64>().unwrap()/65535.;
	    }
	    if i % 3 == 2 && i != 2 {
		op.push(vec3f(r, g, lines[i].parse::<f64>().unwrap()/65535.));
	    }
	}
    }
//    file.write_all(b"P3\n1024 768\n65535\n")?;
//    for i in 0usize..(HEIGHT*WIDTH) as usize {
//	let mut c: geometry::Vec3f = frame_buffer[i];
//	let max = f64::max(c[0], f64::max(c[1], c[2]));
//	if max>1. {c = vec3f(c.x * (1./max), c.y * (1./max), c.z * (1./max))}
//	for j in 0usize..3usize {
//	    if frame_buffer[i][j] < 0. {
//		write!(file, "0")?;
//	    } else if frame_buffer[i][j] > 1. {
//		write!(file, "65535")?;
//	    } else {
//		write!(file, "{}", ((65535 as f64 * frame_buffer[i][j]) as u16).to_string())?;
//	    }
//	    write!(file, " ")?;
//	}
//	write!(file, "\n")?;
//    }
    op
}

fn main() -> std::io::Result<()> {
//    let test1 = geometry::Vec3i{x: 5, y: 5, z: 5};
//    let test2 = geometry::Vec3i{x: 2, y: 2, z: 2};
//    assert!(test1.clone()*test2.clone() == geometry::Vec3i{x: 10, y: 10, z: 10});
//    assert!(test1.clone()+test2.clone() == geometry::Vec3i{x: 7, y: 7, z: 7});
//    assert!(test1.clone()-test2.clone() == geometry::Vec3i{x: 3, y: 3, z: 3});

    let env_file_path = "envmaps/parsed_mountain.ppm";
    let envmap: Vec<geometry::Vec3f> = build_envmap(env_file_path);
//    let _file = save_to_file("envmap_output.ppm", envmap);
    
    let mut spheres: Vec<Sphere> = vec![];
    spheres.push(get_sphere(vec3f(-3., -1.0, -18.), 2., ivory()));
    spheres.push(get_sphere(vec3f(0.5, -1.5, -12.), 2., glass()));
    spheres.push(get_sphere(vec3f(4., -0.5, -18.), 3., red_rubber()));
    spheres.push(get_sphere(vec3f(-14., 9., -19.), 9., mirror()));
    spheres.push(get_sphere(vec3f(0., 8., -22.), 4., mirror()));
    spheres.push(get_sphere(vec3f(0., -1., -25.), 3., red_rubber()));

    let mut lights: Vec<Light> = vec![];
    lights.push(get_light(vec3f(-20., 20., 20.), 1.5));
    lights.push(get_light(vec3f(30., 50., -20.), 1.8));
    lights.push(get_light(vec3f(30., 20., 30.), 1.7));
    
    render(&mut spheres, &mut lights, &envmap);
    Ok(())
}
