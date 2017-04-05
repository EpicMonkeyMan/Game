extern crate glium;
extern crate cgmath;

use opengl;
use matrix;

pub struct Entity {
    pub model_matrix: matrix::Matrix,
    texture: glium::texture::CompressedSrgbTexture2d,
    vertex_buffer: glium::VertexBuffer<opengl::Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: glium::Program
}

#[allow(dead_code)]
impl Entity {
    pub fn new(window: &glium::backend::glutin_backend::GlutinFacade, x: f32, y: f32, z:f32, width: f32, height: f32, rotation: f32, shader_vert: &'static str, shader_frag: &'static str, texture_path: &'static str) -> Entity {
        let texture = opengl::load_opengl_texture(window, texture_path);
        let (vertex_buffer, index_buffer) = opengl::get_quad(window);
        let program = opengl::create_program(window, shader_vert, shader_frag);

        let model_matrix = matrix::Matrix::new(x, y, z, width, height, rotation);

        Entity {
            model_matrix: model_matrix,
            texture: texture,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: program
        }
    }

    pub fn draw(&self, target: &mut glium::Frame, view_matrix: [[f32; 4]; 4], projection_matrix: [[f32; 4]; 4]) {
        use glium::Surface;

        let uniforms = uniform! {
            model: self.model_matrix.get_matrix(),
            view: view_matrix,
            projection: projection_matrix,
            tex: &self.texture
        };
        
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        target.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniforms, &params).unwrap();
    }
}