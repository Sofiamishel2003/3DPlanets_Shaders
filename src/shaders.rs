use nalgebra_glm::{Vec2,Vec3, Vec4, Mat3, dot, mat4_to_mat3,normalize};
use crate::vertex::Vertex;

use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

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
    //time_based_color_cycling_shader(fragment, uniforms)
    sun_shader(fragment, uniforms)
}

pub fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Base colors for the sun effect
  let bright_color = Color::new(255, 240, 0); // Bright orange (lava-like)
  let dark_color = Color::new(130, 20, 0);   // Darker red-orange

  // Get fragment position
  let position = Vec3::new(
    fragment.vertex_position.x,
    fragment.vertex_position.y,
    fragment.depth
  );

  // Base frequency and amplitude for the pulsating effect
  let base_frequency = 0.2;
  let pulsate_amplitude = 0.5;
  let t = uniforms.time as f32 * 0.30;

  // Pulsate on the z-axis to change spot size
  let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

  // Apply noise to coordinates with subtle pulsating on z-axis
  let zoom = 1000.0; // Constant zoom factor
  let noise_value1 = uniforms.noise.get_noise_3d(
    position.x * zoom,
    position.y * zoom,
    (position.z + pulsate) * zoom
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
    (position.x + 1000.0) * zoom,
    (position.y + 1000.0) * zoom,
    (position.z + 1000.0 + pulsate) * zoom
  );
  let noise_value = (noise_value1 + noise_value2) * 0.5;  // Averaging noise for smoother transitions

  // Use lerp for color blending based on noise value
  let color = dark_color.lerp(&bright_color, noise_value);

  color * fragment.intensity
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

pub fn mars_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores base para la superficie de Marte
    let bright_color = Color::new(150, 70, 30);   // Color brillante, como rojo-anaranjado
    let mid_color = Color::new(160, 80, 30);      // Color medio, como marrón
    let dark_color = Color::new(100, 40, 20);     // Color oscuro para áreas sombreadas y cráteres

    // Obtenemos la posición del fragmento
    let position = Vec3::new(
        fragment.vertex_position.x,
        fragment.vertex_position.y,
        fragment.depth,
    );

    // Factor de zoom para mayor detalle en la textura
    let zoom = 1200.0;

    // Generamos ruido para la textura de la superficie de Marte
    let noise_value1 = uniforms.noise.get_noise_3d(position.x * zoom, position.y * zoom, position.z * zoom);
    let noise_value2 = uniforms.noise.get_noise_3d((position.x + 400.0) * zoom, (position.y + 400.0) * zoom, (position.z + 400.0) * zoom);
    let noise_value = (noise_value1 + noise_value2) * 0.5;

    // Añadimos ruido para los cráteres en la superficie
    let crater_frequency = 3.0;  // Aumentamos la frecuencia para más cráteres pequeños
    let crater_amplitude = 0.6;  // Aumentamos la amplitud para hacerlos más evidentes
    let crater_value = (position.x * crater_frequency).sin() * (position.y * crater_frequency).cos() * crater_amplitude;

    // Combinamos el ruido de la superficie y el ruido de los cráteres
    let mut combined_value = (noise_value + crater_value).clamp(0.0, 1.0);

    // Añadimos un ruido fino para más detalles de la superficie
    let fine_noise = uniforms.noise.get_noise_3d(position.x * 2500.0, position.y * 2500.0, position.z * 2500.0) * 0.5;
    combined_value = (combined_value + fine_noise).clamp(0.0, 1.0);

    // Agregamos un ruido para fracturas o detalles más finos
    let fracture_noise = uniforms.noise.get_noise_3d(position.x * 3000.0, position.y * 3000.0, position.z * 3000.0) * 0.3;
    combined_value = (combined_value + fracture_noise).clamp(0.0, 1.0);

    // Determinamos el color de la superficie según el valor combinado
    let base_color = if combined_value > 0.5 {
        mid_color.lerp(&bright_color, (combined_value - 0.5) * 1.5) // Mezclamos con el color brillante si el valor es alto
    } else {
        dark_color.lerp(&mid_color, combined_value * 2.0) // Mezclamos con el color oscuro si el valor es bajo
    };

    // Iluminación difusa (suavizada) para simular la luz sobre la superficie
    let light_factor = (position.y * 0.5 + uniforms.time as f32 * 0.0015).sin() * 0.1 + 1.0;
    let directional_light = (position.x * 0.3 + uniforms.time as f32 * 0.002).cos() * 0.05 + 1.0;
    let final_light_factor = light_factor * directional_light;

    // Aplicamos la luz sobre el color base
    let mut final_color = base_color * final_light_factor;

    // Pulsación en la superficie para dar dinamismo (como la variación de la atmósfera)
    let pulsate_frequency = 0.05;
    let pulsate_amplitude = 0.1;
    let pulsate = (uniforms.time as f32 * pulsate_frequency + position.x * 0.02 + position.y * 0.02).sin() * pulsate_amplitude;
    final_color = final_color * (1.0 + pulsate);

    // Aplicamos una textura de sombra suave con un ruido adicional
    let shadow_texture_noise = uniforms.noise.get_noise_3d(position.x * 3500.0, position.y * 3500.0, position.z * 3500.0) * 0.4;
    final_color = final_color * (1.0 - shadow_texture_noise);

    // Devolvemos el color final multiplicado por la intensidad del fragmento
    final_color * fragment.intensity
}


pub fn mars_shader_wrapper(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    mars_shader(fragment, uniforms) // Simplemente devuelve el Color directamente
}



pub fn earth_shader_wrapper(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let time = uniforms.time as f32; // Usamos el tiempo dinámico que viene de los uniforms

    // Variables para las coordenadas 2D (posición) del fragmento
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    
    // Animación de nubes basada en el tiempo
    let moving_x = x + time * 0.2;  // Velocidad de movimiento en X
    let moving_y = y + time * 0.1;  // Velocidad de movimiento en Y

    // Valores de ruido para la textura de la superficie y para las nubes
    let base_noise_value = uniforms.noise.get_noise_2d(x, y);
    let cloud_noise_value = uniforms.cloud_noise.get_noise_2d(moving_x * 100.0, moving_y * 100.0); // Desplazamiento de nubes

    // Colores base para el agua, tierra y nubes
    let water_color_1 = Color::from_float(0.0, 0.1, 0.6); // Azul oscuro
    let water_color_2 = Color::from_float(0.0, 0.3, 0.7); // Azul claro
    let land_color_1 = Color::from_float(0.1, 0.5, 0.0); // Verde oscuro
    let land_color_2 = Color::from_float(0.2, 0.8, 0.2); // Verde claro
    let cloud_color = Color::from_float(0.9, 0.9, 0.9); // Blanco para las nubes

    let land_threshold = 0.3; // Umbral para determinar si es agua o tierra

    // Determinar el color base del fragmento entre agua y tierra
    let base_color = if base_noise_value > land_threshold {
        // Tierra
        land_color_1.lerp(&land_color_2, (base_noise_value - land_threshold) / (1.0 - land_threshold))
    } else {
        // Agua
        water_color_1.lerp(&water_color_2, base_noise_value / land_threshold)
    };

    // Iluminación difusa (suave) para resaltar la superficie
    let light_position = Vec3::new(1.0, 1.0, 3.0); // Dirección de la luz (sol)
    let light_dir = normalize(&(light_position - fragment.vertex_position)); // Dirección de la luz
    let normal = normalize(&fragment.normal); // Normal del fragmento
    let diffuse = dot(&normal, &light_dir).max(0.0); // Cálculo de la iluminación difusa

    // Aplicar el color base con iluminación difusa
    let lit_color = base_color * (0.1 + 0.9 * diffuse); // Agregar un factor de luz

    // Umbral para las nubes
    let cloud_threshold = 0.1;
    
    let cloud_opacity = 0.8 + 0.2 * ((time / 1000.0) * 0.5).sin().abs(); // Opacidad alta

    // Comprobar si debemos dibujar nubes en este fragmento
    if cloud_noise_value > cloud_threshold {
        let cloud_intensity = ((cloud_noise_value - cloud_threshold) / (1.0 - cloud_threshold)).clamp(0.0, 1.0);
        // Mezclar el color base con las nubes
        return lit_color.blend_add(&(cloud_color * (cloud_intensity * cloud_opacity)));
    } else {
        // No hay nubes, simplemente retornar el color lit
        return lit_color;
    }
}


pub fn jupiter_shader_wrapper(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let base_colors = [
        Vec3::new(0.87, 0.67, 0.44), // Beige marrón
        Vec3::new(0.96, 0.80, 0.69), // Beige claro
        Vec3::new(0.75, 0.50, 0.31), // Marrón oscuro
        Vec3::new(1.00, 0.65, 0.40), // Naranja claro
        Vec3::new(0.95, 0.90, 0.75), // Blanco crema
    ];

    let time = uniforms.time as f32 * 0.001;
    let dynamic_y = fragment.vertex_position.y + time;

    let distortion_scale = 10.0;
    let distortion_value = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * distortion_scale,
        dynamic_y * distortion_scale,
    );

    let distorted_y = dynamic_y + distortion_value * 0.1 + fragment.vertex_position.x * 0.05;

    let band_frequency = 40.0;
    let band_sine = (distorted_y * band_frequency).sin();
    let band_variation = (fragment.vertex_position.y * 10.0).sin() * 0.3;
    let band_index_float = (band_sine + band_variation + 1.0) / 2.0 * (base_colors.len() as f32);
    let band_index = band_index_float as usize % base_colors.len();
    let mut rng = rand::thread_rng();
    let random_offset: f32 = rng.gen_range(-0.03..0.03);
    let base_band_color =
        base_colors[band_index] + Vec3::new(random_offset, random_offset, random_offset);

    // Aumentar la saturación de algunas bandas de forma aleatoria
    let saturation_boost: f32 = if rng.gen_bool(0.5) { 1.2 } else { 1.0 };
    let boosted_band_color = base_band_color * saturation_boost;

    // Se elige el siguiente color de banda para suavizar la transición
    let next_band_index = (band_index + 1) % base_colors.len();
    let next_band_color =
        base_colors[next_band_index] + Vec3::new(random_offset, random_offset, random_offset);

    // Interpolación suave entre colores adyacentes
    let interpolation_factor = band_index_float.fract();
    let interpolated_color = boosted_band_color.lerp(&next_band_color, interpolation_factor);

    // capas de ruido de alta frecuencia para dar más textura a las bandas
    let noise_scale_1 = 80.0;
    let noise_value_1 = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * noise_scale_1,
        fragment.vertex_position.y * noise_scale_1,
    );

    let noise_scale_2 = 40.0;
    let noise_value_2 = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * noise_scale_2,
        fragment.vertex_position.y * noise_scale_2,
    );

    let perturbed_color = interpolated_color * (0.95 + (noise_value_1 + noise_value_2) * 0.015);

    let internal_shadow = (distorted_y * band_frequency * 0.1).sin().abs() * 0.15;
    let shaded_color = perturbed_color * (1.0 - internal_shadow);

    let shadow_noise_scale = 50.0;
    let shadow_noise = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * shadow_noise_scale,
        fragment.vertex_position.y * shadow_noise_scale,
    );
    let shadow_variation = 1.0 - shadow_noise * 0.05;
    let final_shaded_color = shaded_color * shadow_variation;
    let spot_noise_scale = 25.0;
    let spot_noise = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * spot_noise_scale,
        fragment.vertex_position.y * spot_noise_scale,
    );

    let mut final_color;

    if spot_noise > 0.75 {
        let mix_factor = (spot_noise - 0.75) / 0.25;
        let storm_color = Vec3::new(0.95, 0.85, 0.65);
        final_color = final_shaded_color.lerp(&storm_color, mix_factor);
    } else {
        final_color = final_shaded_color;
    }

    let normal = fragment.vertex_position.normalize();

    let light_dir = Vec3::new(0.6, 0.8, 0.4).normalize();
    let lambertian = light_dir.dot(&normal).max(0.0);
    let shading_factor = 0.75 + 0.25 * lambertian;

    final_color = final_color * shading_factor;

    // dispersión atmosférica
    let gradient_shading = 1.0 - (fragment.vertex_position.y.abs() * 0.15);
    final_color = final_color * gradient_shading;

    // reflejos especulares para simular brillos en la atmósfera
    let view_dir = Vec3::new(0.0, 0.0, 1.0).normalize();
    let reflect_dir = (2.0 * normal.dot(&light_dir) * normal - light_dir).normalize();
    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(10.0);

    final_color = final_color + Vec3::new(1.0, 1.0, 1.0) * specular_intensity * 0.15;

    final_color = final_color * fragment.intensity;

    Color::new(
        (final_color.x * 255.0) as u8,
        (final_color.y * 255.0) as u8,
        (final_color.z * 255.0) as u8,
    )
}



pub fn uranus_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let z = fragment.vertex_position.z;
    let t = time as f32 * 0.001; // Scale time for cloud motion

    // Generate smooth noise for atmospheric features
    let noise_value = uniforms.noise.get_noise_3d(x, y + t, z);

    // Base Uranus color
    let base_color = Color::from_float(0.2, 0.5, 0.9); // Light blue for Uranus atmosphere

    // Intensity adjustment for smooth color variation
    let intensity = (noise_value * 0.5 + 0.5).clamp(0.0, 1.0);
    let varied_color = base_color * intensity;

    // Directional lighting for highlights
    let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize();
    let normal = fragment.normal.normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);
    if diffuse_intensity.is_nan() || diffuse_intensity.is_infinite() {
        panic!("Diffuse calculation resulted in NaN or infinity!");
    }
    let ambient_intensity = 0.3; // Base ambient light
    let lit_color = varied_color * (ambient_intensity + (1.0 - ambient_intensity) * diffuse_intensity);

    (lit_color, 0)
}

pub fn uranus_shader_wrapper(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let (color, _extra) = uranus_shader(fragment, uniforms, 0); // Replace `0` with actual `time` if needed
    color
}

pub fn saturn_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> (Color, u32) {
    let latitude = fragment.vertex_position.y;
    let band_frequency = 10.0;

    let band_noise = uniforms.noise.get_noise_2d(
        fragment.vertex_position.x * 2.0,
        fragment.vertex_position.y * 2.0,
    );
    let band_noise_intensity = 0.2;
    let distorted_latitude = latitude + band_noise * band_noise_intensity;
    let band_pattern = (distorted_latitude * band_frequency).sin();

    let band_colors = [
        Color::from_hex(0xc6bcad),
        Color::from_hex(0x955d36),
        Color::from_hex(0xc7c7cf),
    ];

    let normalized_band = (band_pattern + 1.0) / 2.0 * (band_colors.len() as f32 - 1.0);
    let index = normalized_band.floor() as usize;
    let t = normalized_band.fract();
    let color1 = band_colors[index % band_colors.len()];
    let color2 = band_colors[(index + 1) % band_colors.len()];
    let base_color = color1.lerp(&color2, t);

    let turbulence_intensity = 0.3;
    let turbulence_color = base_color.lerp(&Color::from_hex(0xffffff), turbulence_intensity);

    let light_position = Vec3::new(0.0, 8.0, 9.0);
    let light_direction = (light_position - fragment.vertex_position).normalize();
    let normal = fragment.normal.normalize();
    let diffuse = normal.dot(&light_direction).max(0.0);
    if diffuse.is_nan() || diffuse.is_infinite() {
        panic!("Diffuse calculation resulted in NaN or infinity!");
    }

    let ambient_intensity = 0.15;
    let ambient_color = turbulence_color * ambient_intensity;
    let lit_color = turbulence_color * diffuse;

    (ambient_color + lit_color, 0)
}

pub fn saturn_shader_wrapper(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let (color, _extra) = saturn_shader(fragment, uniforms, 0); 
    color
}
pub fn saturn_ring_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let ring_position = Vec2::new(fragment.vertex_position.x, fragment.vertex_position.z);
    let distance_from_center = ring_position.magnitude(); // Distancia radial al centro

    let num_bands = 8; // Más bandas para los anillos
    let max_distance = 1.5; // Tamaño reducido
    let band_width = max_distance / num_bands as f32;

    let band_index = (distance_from_center / band_width).floor() as i32;

    let band_colors = [
        Color::new(225, 190, 160), // Marrón claro
        Color::new(245, 230, 200), // Beige claro
        Color::new(255, 255, 240), // Blanco crema
        Color::new(200, 180, 150), // Marrón pálido
        Color::new(230, 210, 190), // Beige intermedio
    ];

    let base_color = band_colors[(band_index.abs() as usize) % band_colors.len()];

    let edge_distance = (distance_from_center % band_width) / band_width;
    let smooth_edge = (1.0 - edge_distance).clamp(0.0, 1.0);

    let light_position = Vec3::new(1.0, 1.0, 3.0);
    let light_dir = normalize(&(light_position - fragment.vertex_position));
    let normal = normalize(&fragment.normal);
    let diffuse_intensity = dot(&normal, &light_dir).max(0.0);

    let ambient_intensity = 0.3;
    let final_light_factor = ambient_intensity + (1.0 - ambient_intensity) * diffuse_intensity;

    let lit_color = base_color * smooth_edge * final_light_factor;

    let noise = uniforms.noise.get_noise_2d(ring_position.x * 10.0, ring_position.y * 10.0) * 0.1;
    lit_color * (1.0 + noise)
}


pub fn mercury_shader_wrapper(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores base para la superficie rocosa con tonos de gris-dorado
    let bright_color = Color::from_float(0.9, 0.9, 0.9);  // Gris plateado brillante
    let mid_color = Color::from_float(0.6, 0.55, 0.4);     // Gris dorado medio
    let dark_color = Color::from_float(0.3, 0.2, 0.1);     // Gris oscuro con toques cálidos

    // Obtenemos la posición del fragmento
    let position = Vec3::new(fragment.vertex_position.x, fragment.vertex_position.y, fragment.depth);

    // Factor de zoom para mayor detalle en la textura
    let zoom = 1200.0;

    // Generar ruido para la superficie
    let noise_value1 = uniforms.noise.get_noise_3d(position.x * zoom, position.y * zoom, position.z * zoom);
    let noise_value2 = uniforms.noise.get_noise_3d((position.x + 400.0) * zoom, (position.y + 400.0) * zoom, (position.z + 400.0) * zoom);
    let noise_value = (noise_value1 + noise_value2) * 0.5;

    // Parámetros para los cráteres en la superficie (más detalles)
    let crater_frequency = 3.5;  // Aumentar la frecuencia de cráteres para más detalle
    let crater_amplitude = 0.5;  // Aumentar la amplitud de los cráteres para que sean más visibles
    let crater_value = ((position.x * crater_frequency).sin() * (position.y * crater_frequency).cos() * crater_amplitude)
        .clamp(0.0, 1.0);

    // Combinamos el ruido base y el ruido de los cráteres
    let mut combined_value = (noise_value + crater_value).clamp(0.0, 1.0);

    // Añadimos un ruido fino para más textura
    let fine_noise = uniforms.noise.get_noise_3d(position.x * 2500.0, position.y * 2500.0, position.z * 2500.0) * 0.5;
    combined_value = (combined_value + fine_noise).clamp(0.0, 1.0);

    // Agregamos un ruido para las fracturas y detalles más finos
    let fracture_noise = uniforms.noise.get_noise_3d(position.x * 3000.0, position.y * 3000.0, position.z * 3000.0) * 0.3;
    combined_value = (combined_value + fracture_noise).clamp(0.0, 1.0);

    // Determinamos el color base dependiendo del valor combinado
    let base_color = if combined_value > 0.5 {
        mid_color.lerp(&bright_color, (combined_value - 0.5) * 2.0) // Color brillante si el valor es alto
    } else {
        dark_color.lerp(&mid_color, combined_value * 2.0) // Color oscuro si el valor es bajo
    };

    // Iluminación dinámica (ajustar la dirección para que la parte oscura esté en la parte trasera)
    let light_position = Vec3::new(0.0, 0.0, 5.0);  // Luz principal (detrás del planeta)
    let light_direction = (light_position - fragment.vertex_position).normalize(); // Dirección de la luz hacia el planeta
    let normal = fragment.normal.normalize();  // Normal del fragmento
    let diffuse_intensity = normal.dot(&light_direction).max(0.0); // Intensidad de la luz difusa

    // Intensidad ambiental (ajustar para simular más reflexión en las superficies rocosas)
    let ambient_intensity = 0.3;  
    let lit_color = base_color * (ambient_intensity + (1.0 - ambient_intensity) * diffuse_intensity);

    // Aplicar textura de sombra suave con ruido
    let shadow_texture_noise = uniforms.noise.get_noise_3d(
        position.x * 3500.0,
        position.y * 3500.0,
        position.z * 3500.0,
    ) * 0.4;
    let final_color = lit_color * (1.0 - shadow_texture_noise);

    // Pulsación en la superficie para dar más dinamismo
    let pulsate_frequency = 0.05;
    let pulsate_amplitude = 0.08;
    let pulsate = (uniforms.time as f32 * pulsate_frequency + position.x * 0.02 + position.y * 0.02).sin() * pulsate_amplitude;
    
    // Final color modificado por la pulsación
    let final_color = final_color * (1.0 + pulsate);

    // Devolvemos el color final multiplicado por la intensidad del fragmento
    final_color * fragment.intensity
}
pub fn moon_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores base para la luna (grises y tonos tierra)
    let base_gray = Color::from_float(0.7, 0.7, 0.7); // Gris claro
    let dark_gray = Color::from_float(0.4, 0.4, 0.4); // Gris oscuro
    let crater_highlight = Color::from_float(0.8, 0.8, 0.8); // Gris brillante para bordes de cráteres

    // Coordenadas del fragmento
    let position = fragment.vertex_position;
    let noise_scale = 50.0; // Escala para el ruido de cráteres

    // Generar ruido para simular cráteres y variaciones de textura
    let noise_value = uniforms.noise.get_noise_2d(
        position.x * noise_scale,
        position.y * noise_scale,
    );

    let fine_noise = uniforms.noise.get_noise_2d(
        position.x * noise_scale * 2.0,
        position.y * noise_scale * 2.0,
    );

    // Mezclar colores en base al ruido
    let base_color = base_gray.lerp(&dark_gray, noise_value.clamp(0.0, 1.0));

    // Añadir un efecto de borde brillante en los cráteres
    let crater_effect = fine_noise.abs().clamp(0.0, 1.0);
    let crater_color = base_color.lerp(&crater_highlight, crater_effect * 0.5);

    // Iluminación difusa
    let light_position = Vec3::new(2.0, 2.0, 5.0); // Posición de la luz (el "sol")
    let light_dir = normalize(&(light_position - fragment.vertex_position));
    let normal = normalize(&fragment.normal);
    let diffuse = dot(&normal, &light_dir).max(0.0);

    // Combinar iluminación con color base
    let final_color = crater_color * (0.3 + 0.7 * diffuse);

    // Retornar color final
    final_color * fragment.intensity
}
pub fn moon_shader_wrapper(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    moon_shader(fragment, uniforms)
}
