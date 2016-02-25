#[macro_use]
extern crate glium;
extern crate rand;
extern crate image;
extern crate time;

use glium::{DisplayBuild, Surface};
use glium::index::PrimitiveType;
use glium::glutin;

fn main() {
    const SCREEN_W: u32 = 1024;
    const SCREEN_H: u32 = 768;
    const MAP_WIDTH: usize = 32;
    const MAP_HEIGHT: usize = 32;
    const TILE_W: f32 = 12.0;
    const TILE_H: f32 = 12.0;
    const SPRITES_COUNT: usize = MAP_WIDTH * MAP_HEIGHT;

    let display = glutin::WindowBuilder::new()
        .with_dimensions(SCREEN_W, SCREEN_H)
        .build_glium()
        .unwrap();

    let (vertex_buffer, index_buffer) = {
        #[derive(Copy, Clone)]
        struct Vertex {
            i_position: [f32; 2],
            i_color: f32,
        }

        implement_vertex!(Vertex, i_position, i_color);

        let mut vb: glium::VertexBuffer<Vertex> = glium::VertexBuffer::empty_dynamic(&display, SPRITES_COUNT * 4).unwrap();
        let mut ib_data = Vec::with_capacity(SPRITES_COUNT * 6);

        for (num, sprite) in vb.map().chunks_mut(4).enumerate() {
            let color = if num == 0 { 0.5 } else if num % 2 == 0 { 1.0 } else { 0.0 };
            let i = (num % MAP_WIDTH) as f32;
            let j = (num / MAP_WIDTH) as f32;

            let x = i * TILE_W + TILE_W;
            let y = j * TILE_H + TILE_H;

            let position: (f32, f32) = (x, y);

            sprite[0].i_position[0] = position.0 - TILE_W;
            sprite[0].i_position[1] = position.1 + TILE_H;
            sprite[0].i_color = color;
            sprite[1].i_position[0] = position.0 + TILE_W;
            sprite[1].i_position[1] = position.1 + TILE_H;
            sprite[1].i_color = color;
            sprite[2].i_position[0] = position.0 - TILE_W;
            sprite[2].i_position[1] = position.1 - TILE_H;
            sprite[2].i_color = color;
            sprite[3].i_position[0] = position.0 + TILE_W;
            sprite[3].i_position[1] = position.1 - TILE_H;
            sprite[3].i_color = color;

            let num = num as u16;
            ib_data.push(num * 4);
            ib_data.push(num * 4 + 1);
            ib_data.push(num * 4 + 2);
            ib_data.push(num * 4 + 1);
            ib_data.push(num * 4 + 3);
            ib_data.push(num * 4 + 2);
        }

        (vb, glium::index::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &ib_data).unwrap())
    };

    // we determine the texture coordinates depending on the ID the of vertex
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 matrix;

                in vec2 i_position;
                in float i_color;

                out float v_color;

                void main() {
                    gl_Position = matrix * vec4(i_position, 0.0, 1.0);
                    v_color = i_color;
                }
            ",

            fragment: "
                #version 140

                in float v_color;

                out vec4 f_color;

                void main() {
                    f_color = vec4(v_color, 0.0, 0.0, 1.0);
                }
            "
        },
    ).unwrap();

    let mut cnt = 0;
    let mut t0 = time::precise_time_s();

    loop {
        let ib_slice = index_buffer.slice(0 .. SPRITES_COUNT * 6).unwrap();

        let right = SCREEN_W as f32;
        let bottom = SCREEN_H as f32;
        let left = 0.0;
        let top = 0.0;
        let far = 1.0;
        let near = -1.0;

        let uniforms = uniform! {
            matrix: [
                [2.0 / (right - left),             0.0,                              0.0,                          0.0],
                [0.0,                              2.0 / (top - bottom),             0.0,                          0.0],
                [0.0,                              0.0,                              -2.0 / (far - near),          0.0],
                [-(right + left) / (right - left), -(top + bottom) / (top - bottom), -(far + near) / (far - near), 1.0f32],
            ],
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &ib_slice, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                _ => ()
            }
        }

        cnt += 1;
        let t1 = time::precise_time_s();
        let dt = t1 - t0;
        if dt >= 1.0 {
            println!("FPS: {}", cnt);
            cnt = 0;
            t0 = t1;
        }
    };
}
