use std::{ffi::{CStr, CString}, ptr::{null, null_mut}};

use gl::types::{self, GLboolean, GLchar, GLenum, GLint, GLuint};

use super::color::Color;

fn create_cstring(len: usize, character: u8) -> CString {
    let mut buffer = Vec::<u8>::with_capacity(len + 1);
    buffer.extend([character].iter().cycle().take(len));
    unsafe {
        CString::from_vec_unchecked(buffer)
    }
}

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Self, String> {
        let id = unsafe {
            gl::CreateShader(kind) 
        };

        let mut success: GLint = 1;

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);

            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_cstring(len as usize, b' ');
            unsafe {
                gl::GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned())
        }

        Ok(Shader { 
            id
        })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct Program {
    pub id: GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[&Shader]) -> Result<Self, String> {
        let id = unsafe {
            gl::CreateProgram()
        };

        for shader in shaders {
            unsafe {
                gl::AttachShader(id, shader.id);
            }
        }

        let mut success: GLint = 1;

        unsafe {
            gl::LinkProgram(id);
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_cstring(len as usize, b' ');
            unsafe {
                gl::GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned())
        }

        Ok(Program { id })
    }

    pub fn set(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Vbo {
    pub id: GLuint,
}

impl Vbo {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Vbo { id }
    }

    pub fn set(&self, vertices: &Vec<f32>) {
        self.bind();
        self.data(vertices);
    }

    fn data(&self, vertices: &Vec<f32>) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, 
                vertices.as_ptr() as *const gl::types::GLvoid, 
                gl::DYNAMIC_DRAW
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id)
        }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

pub struct Ibo {
    pub id: GLuint,
}

impl Ibo {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Ibo { id }
    }

    pub fn set(&self, indices: &Vec<u32>) {
        self.bind();
        self.data(indices);
    }

    fn data(&self, indices: &Vec<u32>) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, 
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, 
                indices.as_ptr() as *const gl::types::GLvoid, 
                gl::DYNAMIC_DRAW
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id)
        }
    }
}

impl Drop for Ibo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

#[derive(Clone, Copy)]
pub enum VertexArrayElement {
    Integers {
        count: u32
    },
    Floats {
        count: u32,
        normalized: bool
    },
}

impl VertexArrayElement {
    pub fn gl_type(&self) -> u32 {
        match self {
            VertexArrayElement::Integers { .. } => gl::INT,
            VertexArrayElement::Floats { .. } => gl::FLOAT,
        }
    }

    pub fn count(&self) -> u32 {
        match self {
            VertexArrayElement::Integers { count } => *count,
            VertexArrayElement::Floats { count, .. } => *count,
        }
    }

    pub fn element_size(&self) -> u32 {
        match self {
            VertexArrayElement::Integers { .. } => std::mem::size_of::<i32>() as u32,
            VertexArrayElement::Floats { .. } => std::mem::size_of::<f32>() as u32,
        }
    }

    pub fn size(&self) -> u32 {
        self.element_size() * self.count()
    }

    pub fn normalized(&self) -> bool {
        match self {
            VertexArrayElement::Floats { normalized, .. } => *normalized,
            _ => false,
        }
    }
}

pub struct Vao {
    pub id: GLuint,
}

impl Vao {
    pub fn new(format: &[VertexArrayElement]) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
        }

        let stride = format.iter().map(|e| e.size()).sum::<u32>();
        let mut offset: u32 = 0;
        for (index, element) in format.iter().enumerate() {
            unsafe {
                gl::VertexAttribPointer(
                    index as GLuint,
                    element.count() as GLint,
                    element.gl_type() as GLuint,
                    element.normalized() as GLboolean,
                    stride as GLint,
                    offset as *const gl::types::GLvoid
                );
                gl::EnableVertexAttribArray(index as GLuint);
            }
            offset += element.size();
        }

        Vao { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        self.unbind();

        unsafe {
            gl::DeleteVertexArrays(1, &self.id)
        }
    }
}

pub struct Uniform {
    pub id: GLint,
}

impl Uniform {
    pub fn new(program: &Program, name: &str) -> Result<Self, String> {
        let cname = CString::new(name).unwrap();
        let id: GLint = unsafe {
            gl::GetUniformLocation(program.id, cname.as_ptr())
        };

        if id == -1 {
            return Err(format!("Couldn't get uniform location for {}", name));
        }

        Ok(Uniform { id })
    }

    pub fn set1(&self, f1: f32) {
        unsafe {
            gl::Uniform1f(self.id, f1);
        }
    }

    pub fn set2(&self, f1: f32, f2: f32) {
        unsafe {
            gl::Uniform2f(self.id, f1, f2);
        }
    }

    pub fn set3(&self, f1: f32, f2: f32, f3: f32) {
        unsafe {
            gl::Uniform3f(self.id, f1, f2, f3);
        }
    }

    pub fn set4(&self, f1: f32, f2: f32, f3: f32, f4: f32) {
        unsafe {
            gl::Uniform4f(self.id, f1, f2, f3, f4);
        }
    }

    pub fn set_vec3(&self, vec: glm::Vec3) {
        self.set3(vec.x, vec.y, vec.z);
    }

    pub fn set_rgb(&self, color: Color) {
        self.set3(color.r, color.g, color.b);
    }

    pub fn set_mat4(&self, mat: glm::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(self.id, 1, 0, mat.as_ptr() as *const types::GLfloat);
        }
    }
}
