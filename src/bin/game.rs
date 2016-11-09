extern crate game_of_life;
#[macro_use]
extern crate glium;

use game_of_life::simulation::{GameOfLife, Grid};
use glium::{DisplayBuild, Program, Surface, VertexBuffer};
use glium::glutin::ElementState::Pressed;
use glium::glutin::{Event, WindowBuilder, VirtualKeyCode};
use glium::index;
use glium::uniforms::EmptyUniforms;

const FRAMES_PER_CYCLE: u32 = 20;
const GRID_HEIGHT: usize = 20;
const GRID_WIDTH: usize = 20;
const X_OFFSET: usize = 9;
const Y_OFFSET: usize = 9;

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

    let mut passed_frames = 0;
    loop {
        let alive_cells = {
            let mut triangles = Vec::new();

            for x in 0..grid.cols() {
                for y in 0..grid.rows() {
                    if grid[(x, y)] {
                        let x = x as f32;
                        let y = y as f32;

                        triangles.push(Vertex { v_position: convert_coordinates(&grid, (x, y)) });
                        triangles.push(Vertex {
                            v_position: convert_coordinates(&grid, (x + 1.0, y)),
                        });
                        triangles.push(Vertex {
                            v_position: convert_coordinates(&grid, (x, y + 1.0)),
                        });
                        triangles.push(Vertex {
                            v_position: convert_coordinates(&grid, (x, y + 1.0)),
                        });
                        triangles.push(Vertex {
                            v_position: convert_coordinates(&grid, (x + 1.0, y + 1.0)),
                        });
                        triangles.push(Vertex {
                            v_position: convert_coordinates(&grid, (x + 1.0, y)),
                        });
                    }
                }
            }

            VertexBuffer::new(&display, &triangles).unwrap()
        };
        let alive_indices = index::NoIndices(index::PrimitiveType::TrianglesList);

        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
            match ev {
                Event::Closed |
                Event::KeyboardInput(Pressed, _, Some(VirtualKeyCode::Escape)) => return,
                _ => (),
            }
        }

        let mut target = display.draw();
        target.clear_color(0.9, 0.9, 0.9, 1.0);
        target.draw(&alive_cells,
                  &alive_indices,
                  &program,
                  &EmptyUniforms,
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

#[derive(Copy, Clone)]
struct Vertex {
    v_position: [f32; 2],
}

implement_vertex!(Vertex, v_position);
