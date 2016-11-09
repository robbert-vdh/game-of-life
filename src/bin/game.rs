extern crate game_of_life;
#[macro_use]
extern crate glium;

use glium::{DisplayBuild, Program, Surface, VertexBuffer};
use glium::glutin::ElementState::Pressed;
use glium::glutin::{Event, WindowBuilder, VirtualKeyCode};
use glium::index;
use glium::uniforms::EmptyUniforms;

fn main() {
    let display = WindowBuilder::new().build_glium().unwrap();

    let triangle = {
        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [0.0, 0.5] };
        let vertex3 = Vertex { position: [0.5, -0.25] };

        vec![vertex1, vertex2, vertex3]
    };
    let vertex_buffer = VertexBuffer::new(&display, &triangle).unwrap();
    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

    let program = {
        let vertex_shader_src = include_str!("../assets/vs.glsl");
        let fragment_shader_src = include_str!("../assets/fs.glsl");

        Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap()
    };

    let mut x: f32 = 0.0;

    loop {
        x += 0.1;

        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
            match ev {
                Event::Closed |
                Event::KeyboardInput(Pressed, _, Some(VirtualKeyCode::Escape)) => return,
                _ => (),
            }
        }

        let mut target = display.draw();
        target.clear_color(1.0, 0.5, x.sin(), 1.0);
        target.draw(&vertex_buffer,
                  &indices,
                  &program,
                  &EmptyUniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);
