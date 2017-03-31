extern crate glium;
extern crate cgmath;

use opengl;

pub struct Entity {
    x: f32,
    y: f32, 
    width: f32,
    height: f32,
    texture: glium::texture::CompressedSrgbTexture2d,
    vertex_buffer: glium::VertexBuffer<opengl::Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: glium::Program,
    model_matrix: [[f32; 4]; 4],
}

impl Entity {
    pub fn new(window: &glium::backend::glutin_backend::GlutinFacade, x: f32, y: f32, width: f32, height: f32, shader_vert: &'static str, shader_frag: &'static str, texture_path: &'static str) -> Entity {
        let texture = opengl::load_opengl_texture(window, texture_path);
        let (vertex_buffer, index_buffer) = opengl::get_quad(window);
        let program = opengl::create_program(window, shader_vert, shader_frag);

        let scale = 150.0;
        let translate = cgmath::Matrix4::from_translation(cgmath::vec3(scale/2.0+x, scale/2.0+y, 0.0));
        let scale = cgmath::Matrix4::from_scale(scale);
        let model_matrix: [[f32; 4]; 4] = (translate * scale).into();

        Entity {
            x: x,
            y: y,
            width: width, 
            height: height,
            texture: texture,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: program,
            model_matrix: model_matrix,
        }
    }

    pub fn draw(&self, target: &mut glium::Frame, view_matrix: [[f32; 4]; 4], projection_matrix: [[f32; 4]; 4]) {
        use glium::Surface;

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let uniforms = uniform! {
            model: self.model_matrix,
            view: view_matrix,
            projection: projection_matrix,
            tex: &self.texture
        };

        target.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniforms, &params).unwrap();
    }
}