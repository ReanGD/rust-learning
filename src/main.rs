extern crate sdl2;
extern crate rand;
extern crate time;
extern crate cgmath;

mod mesh;
mod scene;
mod tests;
mod device;
mod memory;
mod shader;
mod import3ds;
mod generator;
mod rasterization;

use cgmath::*;
use scene::Scene;
use shader::Shader;
use generator::{generate_plane, generate_sphere};
use import3ds::Loader3ds;


pub fn main() {
    let eye;
    let center;
    let model;
    let ambient;
    let diffuse;
    let ind = 0;
    let mut init_matrix = Matrix4::<f32>::identity();
    let mut add_angle = rad(2.0_f32 * std::f32::consts::PI / 180.0_f32);
    let light = Vector3::new(1.0_f32, -1.0_f32, 1.0_f32);
    match ind {
        0 => {
            eye = Point3::new(0.0_f32, 0.7_f32, -1.1_f32);
	        center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
            ambient = Vector3::<f32>::new(63.059_f32, 50.783_f32, 18.998_f32);
            diffuse = Vector3::<f32>::new(191.668_f32, 154.652_f32, 57.752_f32);
            add_angle = rad(0.0_f32);
            model = Loader3ds::load("ring.3ds").unwrap();
        },
        1 => {
            eye = Point3::new(0.0_f32, 0.0_f32, -10.1_f32);
	        center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
            ambient = Vector3::<f32>::new(0xbb as f32, 0xbb as f32, 0xbb as f32);
            diffuse = Vector3::<f32>::new(0xbb as f32, 0xbb as f32, 0xbb as f32);
            model = Loader3ds::load("tux.3ds").unwrap();
        },
        2 => {
            eye = Point3::new(0.0_f32, -2.0_f32, -4.1_f32);
	        center = Point3::new(0.0_f32, -2.0_f32, 0.0_f32);
            ambient = Vector3::<f32>::new(0xbb as f32, 0xbb as f32, 0xbb as f32);
            diffuse = Vector3::<f32>::new(0xbb as f32, 0xbb as f32, 0xbb as f32);
            init_matrix = Matrix4::from(Matrix3::from_angle_x(rad(-1.8_f32)));
            model = Loader3ds::load("monster.3ds").unwrap();
        },
        3 => {
            eye = Point3::new(0.0_f32, 0.0_f32, 0.5_f32);
	        center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
            ambient = Vector3::<f32>::new(0xbb as f32, 0xbb as f32, 0xbb as f32);
            diffuse = Vector3::<f32>::new(0xbb as f32, 0xbb as f32, 0xbb as f32);
            add_angle = rad(0.0_f32);
            model = generate_plane().unwrap();
        },
        4 => {
            eye = Point3::new(0.0_f32, 0.0_f32, -2.0_f32);
	        center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
            ambient = Vector3::<f32>::new(0xbb as f32, 0xbb as f32, 0xbb as f32);
            diffuse = Vector3::<f32>::new(0xbb as f32, 0xbb as f32, 0xbb as f32);
            model = generate_sphere(50).unwrap();
        },
        
        _ => return
    };
	let up = Vector3::new(0.0_f32, 1.0_f32, 0.0_f32);

    let mut angle = rad(0.0_f32);

    let mut scene = Scene::new(800, 600);
    scene.proj(deg(100.0_f32), 0.1_f32, 100.0_f32)
        .view(eye, center, up)
        .light(light);

    let mut shader = Shader::new(ambient, diffuse, 0.4_f32);
    shader.set_shaders(Shader::vertex_cook_torrance, Shader::pixel_cook_torrance);
    // shader.set_shaders(Shader::vertex_phong_blinn, Shader::pixel_phong_blinn);
    // shader.set_shaders(Shader::vertex_normals, Shader::pixel_normals);
    while scene.start(0xAAAAAA) {
        angle = angle + add_angle;
        scene.draw(&model, Matrix4::from(Matrix3::from_angle_y(angle)) * init_matrix, &mut shader).present();
    }
}
