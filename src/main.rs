#[macro_use]
extern crate glium;
extern crate image;

use glium::{DisplayBuild, Surface};
use glium::index::PrimitiveType;
use glium::glutin;

const MAP_WIDTH: u32 = 32;
const MAP_HEIGHT: u32 = 32;
const TILE_WIDTH: u32 = 10;
const TILE_HEIGHT: u32 = 12;
const TILES_PER_ROW: u32 = 16;

fn main() {
    let display = glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .build_glium()
        .unwrap();

    let tiles = {
        let i = image::open(std::path::Path::new("tiles.png")).unwrap().to_rgba();
        let d = i.dimensions();
        let i = glium::texture::RawImage2d::from_raw_rgba_reversed(i.into_raw(), d);
        glium::texture::Texture2d::new(&display, i).unwrap()
    };

    let (vertex_buffer, index_buffer) = {
        #[derive(Copy, Clone)]
        struct Vertex {
            i_position: [f32; 2],
            i_tex_coords: [f32; 2],
        }

        impl Vertex {
            fn new(pos: [f32; 2], tex: [f32; 2]) -> Vertex {
                Vertex { i_position: pos, i_tex_coords: tex }
            }
        }

        implement_vertex!(Vertex, i_position, i_tex_coords);

        let vb_cnt = (MAP_WIDTH * MAP_HEIGHT * 4) as usize;
        let mut vb_data = Vec::<Vertex>::with_capacity(vb_cnt);

        let ib_cnt = (MAP_WIDTH * MAP_HEIGHT * 6) as usize;
        let mut ib_data = Vec::<u16>::with_capacity(ib_cnt);

        let tile_w = (TILE_WIDTH as f32) / 400.0;
        let tile_h = (TILE_HEIGHT as f32) / 300.0;

        for i in 0..MAP_WIDTH {
            for j in 0..MAP_HEIGHT {
                let tex_id = 1;

                let tu = (tex_id % TILES_PER_ROW) as f32;
                let tv = (tex_id / TILES_PER_ROW) as f32;

                let x = (i * TILE_WIDTH) as f32;
                let y = (j * TILE_HEIGHT) as f32;

                let (x1, y1, u1, v1) = (x - tile_w, y + tile_h, tu * tile_w, tv * tile_h);
                let (x2, y2, u2, v2) = (x + tile_w, y + tile_w, (tu + 1.0) * tile_w, tv * tile_h);
                let (x3, y3, u3, v3) = (x - tile_w, y - tile_w, (tu + 1.0) * tile_w, (tv + 1.0) * tile_h);
                let (x4, y4, u4, v4) = (x + tile_w, y - tile_h, tu * tile_w, (tv + 1.0) * tile_h);

                vb_data.push(Vertex::new([x1, y1], [u1, v1]));
                vb_data.push(Vertex::new([x2, y2], [u2, v2]));
                vb_data.push(Vertex::new([x3, y3], [u3, v3]));
                vb_data.push(Vertex::new([x4, y4], [u4, v4]));

                let n = (i + j * MAP_WIDTH) as u16;
                ib_data.push(n * 4);
                ib_data.push(n * 4 + 1);
                ib_data.push(n * 4 + 2);
                ib_data.push(n * 4 + 1);
                ib_data.push(n * 4 + 3);
                ib_data.push(n * 4 + 2);
            }
        }

        let vb = glium::VertexBuffer::<Vertex>::new(&display, &vb_data).unwrap();
        let ib = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &ib_data).unwrap();

        (vb, ib)
    };

    let vertex_shader_src = r#"
        #version 140

        in vec2 i_position;
        in vec2 i_tex_coords;

        out vec2 v_tex_coords;

        void main() {
            gl_Position = vec4(i_position, 0.0, 1.0);
            v_tex_coords = i_tex_coords;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        uniform sampler2D tex;

        in vec2 v_tex_coords;

        out vec4 f_color;

        void main() {
            f_color = texture2D(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let uniforms = uniform! {
        texture: &tiles,
    };

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for e in display.poll_events() {
            match e {
                glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
