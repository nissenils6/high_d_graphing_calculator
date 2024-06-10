extern crate nalgebra_glm as glm;

pub mod graphics;
pub mod math;

use std::{f32::consts::TAU, ffi::CString};

use gl::types;
use graphics::{camera::Camera, graphics::Graphics, mat3::Mat3, objects::*, vec3::Vec3, winsdl::*};
use sdl2::{event::Event, keyboard::Keycode};

fn f(x: f32, z: f32) -> f32 {
    (x * x + z * z).sqrt().cos()
}

fn main() -> Result<(), String> {
    let mut sdl = Winsdl::new(800, 600, "My window")?;

    let vert_shader = Shader::from_source(&CString::new(include_str!("graphics/.vert")).unwrap(), gl::VERTEX_SHADER)?;
    let frag_shader = Shader::from_source(&CString::new(include_str!("graphics/.frag")).unwrap(), gl::FRAGMENT_SHADER)?;

    let program = Program::from_shaders(&[&vert_shader, &frag_shader])?;
    program.set();

    let vbo = Vbo::new();
    vbo.bind();
    let vao = Vao::new(&[
        VertexArrayElement::Floats { count: 3, normalized: false },
        VertexArrayElement::Floats { count: 3, normalized: false },
        VertexArrayElement::Floats { count: 3, normalized: false },
    ]);
    vao.bind();
    let ibo = Ibo::new();
    ibo.bind();

    let scale = TAU * 2.0;
    let steps = 200.0;

    let mut graphics: Graphics = Graphics::new();
    graphics.surface(-scale, scale, scale / steps, -scale, scale, scale / steps, f);

    let u_world_to_screen = Uniform::new(&program, "u_world_to_screen")?;
    let u_lighting = Uniform::new(&program, "u_lighting")?;

    let mut camera = Camera::new();

    'running: loop {
        for event in sdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if let Some(movement) = match keycode {
                        Keycode::W => Some(glm::Vec3::z()),
                        Keycode::S => Some(-glm::Vec3::z()),
                        Keycode::D => Some(-glm::Vec3::x()),
                        Keycode::A => Some(glm::Vec3::x()),
                        Keycode::E => Some(-glm::Vec3::y()),
                        Keycode::Q => Some(glm::Vec3::y()),
                        _ => None,
                    } {
                        camera.focus += glm::rotate_y_vec3(&(movement * camera.distance * 0.05), camera.horizontal_angle);
                    }
                }
                Event::MouseWheel { y, .. } => {
                    camera.distance = (camera.distance * 0.9f32.powf(y as f32)).clamp(0.2, 25.0);
                }
                Event::MouseMotion { mousestate, xrel, yrel, .. } => {
                    if mousestate.left() {
                        camera.horizontal_angle = (camera.horizontal_angle - xrel as f32 * TAU / 800.0) % TAU;
                        camera.vertical_angle = (camera.vertical_angle + yrel as f32 * TAU / 600.0).clamp(0.0, TAU / 4.0);
                    }
                }
                _ => {}
            }
        }

        program.set();

        u_world_to_screen.set_mat4(camera.matrix());
        u_lighting.set_vec3(glm::Vec3::new(0.0, -1.0, 1.0));

        vbo.set(&graphics.vertex_buffer);
        vao.bind();
        ibo.set(&graphics.index_buffer);

        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);

            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);

            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::DepthMask(gl::FALSE);

            gl::DrawElements(gl::TRIANGLES, graphics.vertices as types::GLsizei, gl::UNSIGNED_INT, 0 as *const _);
        }

        sdl.window.gl_swap_window();
    }

    Ok(())
}
