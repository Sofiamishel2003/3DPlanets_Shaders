use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Transformar la posición del vértice
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );
    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    // División perspectiva
    let w = transformed.w;
    let ndc_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    // Aplicar la matriz de viewport
    let screen_position = uniforms.viewport_matrix * ndc_position;

    // Transformar normales
    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());
    let transformed_normal = normal_matrix * vertex.normal;

    // Crear un nuevo vértice con atributos transformados
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal,
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Shader base (puede ser modificado según el planeta actual)
    time_based_color_cycling_shader(fragment, uniforms)
}

pub fn time_based_color_cycling_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Define una lista de colores para cambiar
    let colors = [
        Color::new(255, 0, 0),    // Rojo
        Color::new(0, 255, 0),    // Verde
        Color::new(0, 0, 255),    // Azul
        Color::new(255, 255, 0),  // Amarillo
        Color::new(255, 0, 255),  // Magenta
        Color::new(0, 255, 255),  // Cian
    ];

    let frames_per_color = 100;
    let color_index = (uniforms.time / frames_per_color) as usize % colors.len();
    let transition_progress = (uniforms.time % frames_per_color) as f32 / frames_per_color as f32;

    let current_color = colors[color_index];
    let next_color = colors[(color_index + 1) % colors.len()];
    current_color.lerp(&next_color, transition_progress) * fragment.intensity
}

pub fn moving_horizontal_stripes_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let color1 = Color::new(255, 0, 0);   // Rojo
    let color2 = Color::new(0, 0, 255);   // Azul
    let stripe_width = 0.2;
    let speed = 0.002;

    let moving_y = fragment.vertex_position.y + uniforms.time as f32 * speed;
    let stripe_factor = ((moving_y / stripe_width) * PI).sin() * 0.5 + 0.5;
    color1.lerp(&color2, stripe_factor) * fragment.intensity
}

pub fn moving_polka_dot_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let background_color = Color::new(250, 250, 250);  // Gris claro
    let dot_color = Color::new(255, 0, 0);             // Rojo

    let dot_size = 0.1;
    let dot_spacing = 0.3;
    let speed = 0.001;

    let moving_x = fragment.vertex_position.x + uniforms.time as f32 * speed;
    let moving_y = fragment.vertex_position.y - uniforms.time as f32 * speed * 0.5;

    let pattern_x = ((moving_x / dot_spacing) * 2.0 * PI).cos();
    let pattern_y = ((moving_y / dot_spacing) * 2.0 * PI).cos();
    let dot_pattern = (pattern_x * pattern_y).max(0.0);

    let dot_factor = (dot_pattern - (1.0 - dot_size)).max(0.0) / dot_size;
    background_color.lerp(&dot_color, dot_factor) * fragment.intensity
}

pub fn disco_ball_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let base_color = Color::new(100, 100, 210);
    let light_color = Color::new(255, 255, 255);

    let tile_freq_x = 20.0;
    let tile_freq_y = 40.0;
    let tile_freq_z = 20.0;
    let tile_scale = 0.05;

    let light_speed = 0.05;
    let num_lights = 5;
    let light_size = 0.15;

    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let z = fragment.vertex_position.z;

    let tile_pattern = (
        ((x * tile_freq_x).sin().abs() * 0.5 + 0.5) * 
        ((y * tile_freq_y).sin().abs() * 0.8 + 0.2) * 
        ((z * tile_freq_z).sin().abs() * 0.5 + 0.5) * 
        tile_scale
    ).min(1.0);

    let normal = fragment.normal.normalize();
    let light_dir = Vec3::new(0.0, 0.0, -1.0);
    let light_intensity = dot(&normal, &light_dir).max(0.0);

    let mut light_factor = 0.0;
    for i in 0..num_lights {
        let angle = 2.0 * PI * (i as f32 / num_lights as f32) + uniforms.time as f32 * light_speed;
        let light_x = (angle.cos() * 0.5 + 0.5) * 0.8 + 0.1;
        let light_y = (angle.sin() * 0.5 + 0.5) * 0.8 + 0.1;
        
        let dist = ((x - light_x).powi(2) + (y - light_y).powi(2)).sqrt();
        light_factor += (1.0 - (dist / light_size).min(1.0)).max(0.0);
    }
    light_factor = light_factor.min(1.0);

    let tile_color = base_color.lerp(&light_color, tile_pattern * light_intensity);
    tile_color.lerp(&light_color, light_factor * 0.7) * fragment.intensity
}
