use crate::graphics::{vec3::*, color::*, mat3::*};

pub struct Graphics {
    vertex_buffer: Vec<f32>,
    // order of magnitude at around 1 million objects in the vertex buffer
    // ~   4 MB
    index_buffer: Vec<i32>,
    // -:   :- 100k
    // ~ 400 kB
    vertices: i32,
}

impl Graphics {
    pub fn new() -> Self {
        Graphics {
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            vertices: 0,
        }
    }

    pub fn vertex(&mut self, position: Vec3, color: Color, normal: Vec3) -> i32 {
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
}