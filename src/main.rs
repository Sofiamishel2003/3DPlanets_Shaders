use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
use crate::color::Color;
use crate::fragment::Fragment;
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{vertex_shader, sun_shader, moon_shader, mars_shader, fragment_shader, time_based_color_cycling_shader, moving_horizontal_stripes_shader,
              moving_polka_dot_shader, disco_ball_shader};

pub struct UniformsPlanet {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite,
}

pub struct UniformsMoon {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite,
}

// Noises ---------------------------------------------------------------------------------------------------------
fn create_noise() -> FastNoiseLite {
    create_sun_noise()
}

fn create_mars_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(5678); 
    noise.set_noise_type(Some(NoiseType::Perlin));  
    noise.set_fractal_type(Some(FractalType::FBm)); 
    noise.set_fractal_octaves(Some(6)); 
    noise.set_fractal_lacunarity(Some(2.0));  
    noise.set_fractal_gain(Some(0.5));  
    noise.set_frequency(Some(0.005));  

    noise
}

fn create_sun_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(Some(6));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(0.002));
    noise
}

// View ------------------------------------------------------------------------------------------------------------
fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {  // Eliminar aspect_ratio
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 70.0 * PI / 180.0; 
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &UniformsPlanet, vertex_array: &[Vertex], planet_shader: fn(&Fragment, &UniformsPlanet) -> Color) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            let color = planet_shader(&fragment, &uniforms);
            framebuffer.set_current_color(color.to_hex());
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

// Función para calcular la posición orbital de la luna
fn calculate_moon_position(time: u32, distance: f32, speed: f32) -> Vec3 {
    let angle = time as f32 * speed;  
    let x = distance * angle.cos();  
    let z = distance * angle.sin();  

    Vec3::new(x, 0.0, z)  
}

// Main -------------------------------------------------------------------------------------------------------------------------------------
fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Renderer Example",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x333355);

    let translation = Vec3::new(0.0, 0.0, 0.0);
    let rotation = Vec3::new(0.0, 0.0, 0.0);
    let scale = 2.0f32;

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let obj = Obj::load("assets/sphere.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array(); 
    let mut time = 0;
    let mut current_planet = 1;
     // Parámetros de la luna
    let moon_scale = 0.5;   
    let moon_distance = 2.5;
    let moon_orbit_speed = 0.001; 
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        time += 1;

        match window.get_keys().last() {
            Some(Key::Key1) => current_planet = 1,
            Some(Key::Key2) => current_planet = 2,
            Some(Key::Key3) => current_planet = 3,
            Some(Key::Key4) => current_planet = 4,
            Some(Key::Key5) => current_planet = 5,
            Some(Key::Key6) => current_planet = 6,
            Some(Key::Key7) => current_planet = 7,
            _ => (),
        }

        handle_input(&window, &mut camera);

        framebuffer.clear();
        let noise = match current_planet {
            1 => create_sun_noise(),
            2 => create_mars_noise(),
            _ => FastNoiseLite::with_seed(0),
        };
        let aspect_ratio = window_width as f32 / window_height as f32;
        let model_matrix = create_model_matrix(translation, scale, rotation);
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

        let uniforms_planet = UniformsPlanet { 
            model_matrix, 
            view_matrix, 
            projection_matrix, 
            viewport_matrix, 
            time, 
            noise
        };

        let planet_shader = match current_planet {
            1 => sun_shader,
            2 => mars_shader,
            3 => moving_horizontal_stripes_shader,
            4 => moving_polka_dot_shader,
            5 => disco_ball_shader,
            _ => time_based_color_cycling_shader,
        };

        render(&mut framebuffer, &uniforms_planet, &vertex_arrays, planet_shader);

        if current_planet == 2 {
            let moon_position = calculate_moon_position(time, moon_distance, moon_orbit_speed);
            let moon_translation = moon_position;
            let moon_model_matrix = create_model_matrix(moon_translation, moon_scale, Vec3::new(0.0, 0.0, 0.0));

            let uniforms_moon = UniformsMoon {
                model_matrix: moon_model_matrix,
                view_matrix: view_matrix,
                projection_matrix: projection_matrix,
                viewport_matrix: viewport_matrix,
                time: time,
                noise: FastNoiseLite::with_seed(42),
            };

            render(&mut framebuffer, &uniforms_moon, &vertex_arrays, moon_shader);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
    }
}

fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI / 50.0;
    let zoom_speed = 0.1;

    if window.is_key_down(Key::Left) {
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
        camera.orbit(0.0, rotation_speed);
    }

    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
        movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
    }

    if window.is_key_down(Key::Up) {
        camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
        camera.zoom(-zoom_speed);
    }
}
