use std::ffi::CString;

use crate::graphics::color::*;
use crate::graphics::objects::*;

use super::camera::Camera;

pub struct Graphics3D {
    pub vertex_buffer: Vec<f32>,
    pub index_buffer: Vec<u32>,
    pub vertices: u32,

    vert_shader: Shader,
    frag_shader: Shader,
    program: Program,
    vbo: Vbo,
    vao: Vao,
    ibo: Ibo,
    u_world_to_screen: Uniform,
    u_lighting: Uniform,
}

impl Graphics3D {
    pub fn new() -> Result<Self, String> {
        let vert_shader = Shader::from_source(&CString::new(include_str!("./graphics3d.vert")).unwrap(), gl::VERTEX_SHADER)?;
        let frag_shader = Shader::from_source(&CString::new(include_str!("./graphics3d.frag")).unwrap(), gl::FRAGMENT_SHADER)?;

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

        let u_world_to_screen = Uniform::new(&program, "u_world_to_screen")?;
        let u_lighting = Uniform::new(&program, "u_lighting")?;

        Ok(Graphics3D {
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            vertices: 0,

            vert_shader,
            frag_shader,
            program,
            vbo,
            vao,
            ibo,
            u_world_to_screen,
            u_lighting,
        })
    }

    pub fn clear(&mut self) {
        self.vertex_buffer.clear();
        self.index_buffer.clear();
        self.vertices = 0;
    }

    pub fn vertex(&mut self, position: glm::Vec3, color: Color, normal: glm::Vec3) -> u32 {
        let id = self.vertices;
        self.vertices += 1;
        self.vertex_buffer.push(position.x);
        self.vertex_buffer.push(position.y);
        self.vertex_buffer.push(position.z);
        self.vertex_buffer.push(color.r);
        self.vertex_buffer.push(color.g);
        self.vertex_buffer.push(color.b);
        self.vertex_buffer.push(normal.x);
        self.vertex_buffer.push(normal.y);
        self.vertex_buffer.push(normal.z);
        id
    }

    pub fn triangle(&mut self, p0: glm::Vec3, p1: glm::Vec3, p2: glm::Vec3, color: Color) {
        let normal = glm::normalize(&(p1 - p0).cross(&(p2 - p0)));
        let i0 = self.vertex(p0, color, normal);
        let i1 = self.vertex(p1, color, normal);
        let i2 = self.vertex(p2, color, normal);

        self.index_buffer.push(i0);
        self.index_buffer.push(i1);
        self.index_buffer.push(i2);
    }

    pub fn quad(&mut self, p0: glm::Vec3, p1: glm::Vec3, p2: glm::Vec3, p3: glm::Vec3, color: Color) {
        let normal = glm::normalize(&(p1 - p0).cross(&(p2 - p0)));
        let i0 = self.vertex(p0, color, normal);
        let i1 = self.vertex(p1, color, normal);
        let i2 = self.vertex(p2, color, normal);
        let i3 = self.vertex(p3, color, normal);

        self.index_buffer.push(i0);
        self.index_buffer.push(i1);
        self.index_buffer.push(i2);
        self.index_buffer.push(i2);
        self.index_buffer.push(i3);
        self.index_buffer.push(i0);
    }

    pub fn surface(&mut self, x_min: f32, x_max: f32, x_step: f32, z_min: f32, z_max: f32, z_step: f32, f: fn(f32, f32) -> f32) {
        let color = Color::from_rgb(0.4, 0.0, 0.6);

        let mut x = x_min;
        while x < x_max {
            let mut z = z_min;
            while z < z_max {
                let p00 = glm::Vec3::new(x, f(x, z), z);
                let p01 = glm::Vec3::new(x, f(x, z + z_step), z + z_step);
                let p10 = glm::Vec3::new(x + x_step, f(x + x_step, z), z);
                let p11 = glm::Vec3::new(x + x_step, f(x + x_step, z + z_step), z + z_step);

                self.triangle(p00, p01, p11, color);
                self.triangle(p11, p10, p00, color);

                z += z_step;
            }
            x += x_step;
        }
    }

    pub fn render(&mut self, camera: &Camera) {
        self.program.set();

        self.u_world_to_screen.set_mat4(camera.matrix());
        self.u_lighting.set_vec3(glm::Vec3::new(0.0, -1.0, 1.0));

        self.vao.bind();
        self.vbo.set(&self.vertex_buffer);
        self.ibo.set(&self.index_buffer);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::DepthMask(gl::TRUE);

            gl::DrawElements(gl::TRIANGLES, self.vertices as gl::types::GLsizei, gl::UNSIGNED_INT, 0 as *const _);
        }
    }
}
