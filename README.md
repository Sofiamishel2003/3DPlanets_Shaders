# 3D Planetary Renderer Shaders 🌌

Este es un proyecto de renderización gráfica desarrollado en **Rust**, que simula la visualización de planetas del sistema solar junto con sus características visuales, utilizando **shaders** personalizados y técnicas avanzadas de manipulación de vértices y fragmentos. El proyecto también incluye anillos dinámicos y simulaciones de texturas basadas en ruido para crear efectos visuales realistas.

## 🚀 Características

- **Renderización de planetas con shaders personalizados:** Cada planeta tiene un shader único que define su textura, colores y efectos visuales.
- **Simulación de anillos:** Los anillos de Saturno tienen texturas personalizadas y bordes suaves.
- **Cámara orbital:** Permite moverse y hacer zoom alrededor de los planetas.
- **Texturas basadas en ruido:** Se utiliza la librería `FastNoiseLite` para generar patrones complejos.
- **Modelos OBJ:** Los planetas y anillos son renderizados desde archivos `.obj`.

## 🛠️ Tecnologías utilizadas

- **Rust**: Lenguaje principal para el desarrollo.
- **nalgebra_glm**: Librería matemática para manejar transformaciones y vectores 3D.
- **minifb**: Librería para manejar ventanas y buffers gráficos.
- **FastNoiseLite**: Generador de ruido para texturas procedurales.

## 📂 Estructura del proyecto

```
📁 src/
├── camera.rs         # Manejo de la cámara
├── color.rs          # Representación de colores y operaciones
├── fragment.rs       # Manejo de fragmentos en shaders
├── framebuffer.rs    # Buffer de píxeles para renderización
├── line.rs           # Renderización de líneas
├── main.rs           # Punto de entrada del programa
├── obj.rs            # Carga de modelos OBJ
├── shaders.rs        # Shaders personalizados para planetas y anillos
├── triangle.rs       # Renderización de triángulos
└── vertex.rs         # Manejo de vértices y transformaciones
```

## 🎮 Controles

- **Movimiento de la cámara:**
  - Flechas izquierda/derecha: Rotar alrededor del planeta.
  - Flechas arriba/abajo: Mover la cámara en el eje vertical.
- **Zoom:**
  - Flecha arriba: Acercar.
  - Flecha abajo: Alejar.
- **Cambio de planetas:**
  - Teclas del `1` al `7`: Cambia entre los planetas disponibles.

## 🖼️ Galería de planetas

1. **Sol** 🌞 (estrella)
   - Descripción: Textura dinámica simulando el brillo solar con patrones pulsantes.
   - ![Planetas (7)](https://github.com/user-attachments/assets/057846fc-d3ab-403d-812c-4fe5a2199e3b)


2. **Marte** 🔴 (planeta rocoso)
   - Descripción: Superficie rocosa con tonos rojizos y cráteres.
   - ![image](https://github.com/user-attachments/assets/e2f2cb67-dbda-4aeb-86ea-c78693bfa897)

3. **Tierra** 🌍 (planeta rocoso)
   - Descripción: Representación con océanos, continentes y animación de nubes.
   - ![Planetas (8)](https://github.com/user-attachments/assets/9dbad704-babf-49b3-9c08-93185a586c83)

4. **Júpiter** 🟠(gigante gaseoso)
   - Descripción: Bandas de gases en tonos anaranjados y beige.
   - ![image](https://github.com/user-attachments/assets/3aedc07d-40de-4d5d-bf71-25795ccdd56e)

5. **Mercurio** ⚪(planeta rocoso)
   - Descripción: Superficie gris metálico con cráteres.
   - ![image](https://github.com/user-attachments/assets/2505133a-2d50-4d0e-9e68-0c823fcb5aea)

6. **Urano** 🟦 (gigante gaseoso)
   - Descripción: Tono azul claro con efectos de atmósfera gaseosa.
   - ![image](https://github.com/user-attachments/assets/94a4a09d-9349-462c-aed3-0b06a2c29a5f)

7. **Saturno** 🪐(gigante gaseoso)
   - Descripción: Bandas de gases similares a Júpiter con anillos dinámicos.
   - ![image](https://github.com/user-attachments/assets/580d43e6-3ace-4161-987c-10f46ba9982b)



## 🔧 Instalación y uso

1. Clona el repositorio:
   ```bash
   git clone https://github.com/tu-usuario/planetary-renderer.git
   cd planetary-renderer
   ```

2. Compila y ejecuta:
   ```bash
   cargo run
   ```

3. Disfruta explorando el sistema solar 🌌.


