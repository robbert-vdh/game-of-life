extern crate game_of_life;
#[macro_use]
extern crate glium;

use glium::{DisplayBuild, Program, Surface, VertexBuffer};
use glium::backend::Facade;
use glium::glutin::ElementState::Pressed;
use glium::glutin::{Event, WindowBuilder, VirtualKeyCode};
use glium::index;
use glium::texture::Texture2d;
use glium::uniforms::{EmptyUniforms, MagnifySamplerFilter};

use game_of_life::simulation::{GameOfLife, Grid};

const FRAMES_PER_CYCLE: u32 = 20;
const GRID_HEIGHT: usize = 20;
const GRID_WIDTH: usize = 20;
const X_OFFSET: usize = 10;
const Y_OFFSET: usize = 2;

fn main() {
    let display = WindowBuilder::new().build_glium().unwrap();

    let mut grid = Grid::new(GRID_HEIGHT, GRID_WIDTH);
    for point in &[(X_OFFSET + 1, Y_OFFSET + 1),
                   (X_OFFSET, Y_OFFSET + 2),
                   (X_OFFSET + 1, Y_OFFSET + 2),
                   (X_OFFSET + 2, Y_OFFSET + 2),
                   (X_OFFSET, Y_OFFSET + 3),
                   (X_OFFSET + 2, Y_OFFSET + 3),
                   (X_OFFSET + 1, Y_OFFSET + 4)] {
        grid[*point] = true;
    }

    let program = {
        let vertex_shader_src = include_str!("../assets/vs.glsl");
        let fragment_shader_src = include_str!("../assets/fs.glsl");

        Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap()
    };

    let post_program = {
        let vertex_shader_src = include_str!("../assets/vs.glsl");
        let fragment_shader_src = include_str!("../assets/fs_post.glsl");

        Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap()
    };

    let mut passed_frames = 0;
    loop {
        let (alive_cells, alive_indices) = render_grid(&display, &grid);

        for ev in display.poll_events() {
            match ev {
                Event::Closed |
                Event::KeyboardInput(Pressed, _, Some(VirtualKeyCode::Escape)) => return,
                _ => (),
            }
        }

        let (width, height) = display.get_framebuffer_dimensions();
        let texture = Texture2d::empty(&display, width, height).unwrap();
        {
            let mut render_target = texture.as_surface();
            render_target.clear_color(0.9, 0.9, 0.9, 1.0);
            render_target.draw(&alive_cells,
                      &alive_indices,
                      &program,
                      &EmptyUniforms,
                      &Default::default())
                .unwrap();
        }

        let (post_quad, post_indices) = create_quad(&display);
        let post_uniforms = uniform! {
            screen_texture: texture.sampled().magnify_filter(MagnifySamplerFilter::Nearest)
        };

        let mut target = display.draw();
        target.clear_color(0.9, 0.9, 0.9, 1.0);
        target.draw(&post_quad,
                  &post_indices,
                  &post_program,
                  &post_uniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();

        passed_frames += 1;
        if passed_frames > FRAMES_PER_CYCLE {
            grid = grid.simulate();
            passed_frames = 0;
        }
    }
}

/// Convert coordinates from a `Gridn` to screen coordinates, assuming OpenGL's default coordinate
/// system.
fn convert_coordinates(grid: &Grid, (x, y): (f32, f32)) -> [f32; 2] {
    [x / grid.cols() as f32 * 2.0 - 1.0, y / grid.rows() as f32 * -2.0 + 1.0]
}

/// Prepare the grid for redering.
fn render_grid<F: Facade>(display: &F, grid: &Grid) -> (VertexBuffer<Vertex>, index::NoIndices) {
    let alive_cells = {
        let mut triangles = Vec::new();

        for x in 0..grid.cols() {
            for y in 0..grid.rows() {
                if grid[(x, y)] {
                    let x = x as f32;
                    let y = y as f32;

                    triangles.push(Vertex { v_position: convert_coordinates(grid, (x, y)) });
                    triangles.push(Vertex { v_position: convert_coordinates(grid, (x + 1.0, y)) });
                    triangles.push(Vertex { v_position: convert_coordinates(grid, (x, y + 1.0)) });
                    triangles.push(Vertex { v_position: convert_coordinates(grid, (x, y + 1.0)) });
                    triangles.push(Vertex {
                        v_position: convert_coordinates(grid, (x + 1.0, y + 1.0)),
                    });
                    triangles.push(Vertex { v_position: convert_coordinates(grid, (x + 1.0, y)) });
                }
            }
        }

        VertexBuffer::new(display, &triangles).unwrap()
    };

    (alive_cells, index::NoIndices(index::PrimitiveType::TrianglesList))
}

/// Create the screen quad on which post processing will take place.
fn create_quad<F: Facade>(display: &F) -> (VertexBuffer<Vertex>, index::NoIndices) {
    let vbo = {
        let quad = vec![Vertex { v_position: [-1.0, -1.0] },
                        Vertex { v_position: [1.0, -1.0] },
                        Vertex { v_position: [1.0, 1.0] },
                        Vertex { v_position: [-1.0, 1.0] }];

        VertexBuffer::new(display, &quad).unwrap()
    };

    (vbo, index::NoIndices(index::PrimitiveType::TriangleFan))
}

#[derive(Copy, Clone)]
struct Vertex {
    v_position: [f32; 2],
}

implement_vertex!(Vertex, v_position);
