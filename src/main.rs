pub mod graphics;

use std::{f64::consts::PI, ffi::CString};

use graphics::{winsdl::*, objects::*};
use sdl2::event::Event;

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
        Complex {
            a: r * theta.cos(),
            b: r * theta.sin(),
        }
    }

    fn abs(self) -> f64 {
        ((self.a * self.a) + (self.b * self.b)).sqrt()
    }

    fn arg(self) -> f64 {
        (self.b.atan2(self.a) + 2.0 * PI) % (2.0 * PI)
    }

    fn con(self) -> Complex {
        Complex {
            a: self.a,
            b: -self.b,
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Complex {
        Complex {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            a: (self.a * rhs.a) - (self.b * rhs.b),
            b: (self.a * rhs.b) + (self.b * rhs.a),
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Complex {
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

// TEST

fn main() -> Result<(), String> {
    let mut sdl = Winsdl::new(800, 600, "My window")?;

    let vert_shader = Shader::from_source(&CString::new(include_str!("graphics/.vert")).unwrap(), gl::VERTEX_SHADER)?;
    let frag_shader = Shader::from_source(&CString::new(include_str!("graphics/.frag")).unwrap(), gl::FRAGMENT_SHADER)?;
    
    let program = Program::from_shaders(&[&vert_shader, &frag_shader])?;
    program.set();
    
    let vbo = Vbo::new();
    vbo.bind();
    let vao = Vao::new(&[
        VertexArrayElement::Floats { count: 2, normalized: false },
        VertexArrayElement::Floats { count: 3, normalized: false },
    ]);
    vao.bind();
    let ibo = Ibo::new();
    ibo.bind();

    let vertices = vec![
        -0.5, -0.5, 1.0, 0.0, 0.0,
        -0.5,  0.5, 0.0, 1.0, 0.0,
         0.5,  0.5, 0.0, 0.0, 1.0,
         0.5, -0.5, 1.0, 1.0, 1.0,
    ];

    let indices = vec![
        0, 1, 2, 
        2, 3, 0,
    ];

    let u_resolution = Uniform::new(&program, "u_resolution")?;
    let u_camera_position = Uniform::new(&program, "u_camera_position")?;
    let u_camera_scale = Uniform::new(&program, "u_camera_scale")?;

    'running: loop {
        for event in sdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        program.set();

        vbo.set(&vertices);
        vao.bind();
        ibo.set(&indices);

        u_resolution.set2(800.0, 600.0);
        u_camera_position.set2(0.0, 0.0);
        u_camera_scale.set1(1.0);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }


        sdl.window.gl_swap_window();
    }

    Ok(())
}
