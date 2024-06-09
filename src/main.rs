pub mod graphics;

use std::{f32::consts::{PI, TAU}, ffi::CString};

use gl::types;
use graphics::{graphics::Graphics, mat3::Mat3, objects::*, vec3::Vec3, winsdl::*};
use sdl2::{event::Event, keyboard::Keycode, sys::KeyCode};

#[derive(Clone, Copy)]
struct Complex {
    // z = a + bi
    a: f64,
    b: f64,
}

impl Complex {
    fn new(a: f64, b: f64) -> Complex {
        Complex { a, b }
    }

    fn from_polar(r: f64, theta: f64) -> Complex {
        Complex { a: r * theta.cos(), b: r * theta.sin() }
    }

    fn abs(self) -> f64 {
        ((self.a * self.a) + (self.b * self.b)).sqrt()
    }

    fn arg(self) -> f64 {
        (self.b.atan2(self.a) + std::f64::consts::TAU) % (std::f64::consts::TAU)
    }

    fn con(self) -> Complex {
        Complex { a: self.a, b: -self.b }
    }

    fn pow(self, v: f64) -> Complex {
        let ang: f64 = self.arg() * v;
        let rad: f64 = self.abs().powf(v);
        Complex::from_polar(rad, ang)
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex { a: self.a + rhs.a, b: self.b + rhs.b }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Complex { a: self.a - rhs.a, b: self.b - rhs.b }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Complex {
            a: (self.a * rhs.a) - (self.b * rhs.b),
            b: (self.a * rhs.b) + (self.b * rhs.a),
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let den: f64 = (rhs.a * rhs.a) + (rhs.b * rhs.b);
        Complex {
            a: ((self.a * rhs.a) + (self.b * rhs.b)) / (den),
            b: ((self.b * rhs.a) + (self.a * rhs.b)) / (den),
        }
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.a, self.b)
    }
}

fn f(x: f32, z: f32) -> f32 {
    (x * x + z * z)
}

fn construct_camera_orientation(camera_focus: Vec3, camera_zoom: f32, vertical_angle: f32, horizontal_angle: f32) -> (Vec3, Mat3) {
    let position = camera_focus + (Mat3::rot_y(horizontal_angle) * Mat3::rot_x(vertical_angle) * (Vec3::z(-camera_zoom)));
    let matrix = Mat3::rot_x(-vertical_angle) * Mat3::rot_y(-horizontal_angle);
    (position, matrix)
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

    let mut graphics: Graphics = Graphics::new();
    graphics.surface(-0.5, 0.5, 0.1, -0.5, 0.5, 0.1, f);

    let u_resolution = Uniform::new(&program, "u_resolution")?;
    let u_camera_position = Uniform::new(&program, "u_camera_position")?;
    let u_camera_orientation = Uniform::new(&program, "u_camera_orientation")?;
    let u_lighting = Uniform::new(&program, "u_lighting")?;

    let mut camera_focus: Vec3 = Vec3::ZERO;
    let mut camera_zoom: f32 = 0.2;
    let mut vertical_angle: f32 = TAU / 8.0;
    let mut horizontal_angle: f32 = TAU / 8.0;

    'running: loop {
        for event in sdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if let Some(movement) = match keycode {
                        Keycode::W => Some(Vec3::Z),
                        Keycode::S => Some(-Vec3::Z),
                        Keycode::D => Some(Vec3::X),
                        Keycode::A => Some(-Vec3::X),
                        Keycode::E => Some(Vec3::Y),
                        Keycode::Q => Some(-Vec3::Y),
                        _ => None,
                    } {
                        camera_focus = camera_focus + Mat3::rot_y(horizontal_angle) * (movement * camera_zoom * 0.05);
                    }
                }
                Event::MouseWheel { timestamp, window_id, which, x, y, direction, precise_x, precise_y } => {
                    camera_zoom += y as f32 * 0.1;
                    println!("{}", camera_zoom);
                }
                Event::MouseMotion { mousestate, xrel, yrel, .. } => {
                    if mousestate.left() {
                        horizontal_angle += xrel as f32 * TAU / 800.0;
                        vertical_angle = (vertical_angle + yrel as f32 * TAU / 600.0).clamp(0.0, TAU / 4.0);
                    }
                }
                _ => {}
            }
        }

        program.set();

        let (camera_position, camera_orientation) = construct_camera_orientation(camera_focus, camera_zoom, vertical_angle, horizontal_angle);

        u_resolution.set2(800.0, 600.0);
        u_camera_position.set_vec3(camera_position);
        u_camera_orientation.set_mat3(camera_orientation);
        u_lighting.set_vec3(Vec3::new(0.0, -1.0, 0.0));

        vbo.set(&graphics.vertex_buffer);
        vao.bind();
        ibo.set(&graphics.index_buffer);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(gl::TRIANGLES, graphics.vertices as types::GLsizei, gl::UNSIGNED_INT, 0 as *const _);
        }

        sdl.window.gl_swap_window();
    }

    Ok(())
}
