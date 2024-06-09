use crate::graphics::{vec3::*, color::*, mat3::*};

pub struct Graphics {
    pub vertex_buffer: Vec<f32>,
    // order of magnitude at around 1 million objects in the vertex buffer
    // ~   4 MB
    pub index_buffer: Vec<u32>,
    // -:   :- 100k
    // ~ 400 kB
    pub vertices: u32,
}

impl Graphics {
    pub fn new() -> Self {
        Graphics {
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            vertices: 0,
        }
    }

    pub fn vertex(&mut self, position: Vec3, color: Color, normal: Vec3) -> u32 {
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
        
    pub fn triangle(&mut self, p0: Vec3, p1: Vec3, p2: Vec3, color: Color) {
        let normal: Vec3 = (p1 - p0).cross(p2 - p0).unit();
        let i0 = self.vertex(p0, color, normal);
        let i1 = self.vertex(p1, color, normal);
        let i2 = self.vertex(p2, color, normal);

        self.index_buffer.push(i0);
        self.index_buffer.push(i1);
        self.index_buffer.push(i2);
    }

    pub fn quad(&mut self, p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, color: Color) {
        let normal: Vec3 = (p1 - p0).cross(p2 - p0).unit();
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
        let color = Color::from_rgb(0.5, 0.5, 0.5);

        let mut x = x_min;
        while x < x_max {
            let mut z = z_min;
            while z < z_max {
                let p00 = Vec3::new(x, f(x, z), z);
                let p01 = Vec3::new(x, f(x, z + z_step), z + z_step);
                let p10 = Vec3::new(x + x_step, f(x + x_step, z), z);
                let p11 = Vec3::new(x + x_step, f(x + x_step, z + z_step), z + z_step);

                self.triangle(p00, p01, p11, color);
                self.triangle(p11, p10, p00, color);
    
                z += z_step;
            }
            x += x_step;
        }
    }
}