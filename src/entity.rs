extern crate glium;
extern crate cgmath;

use opengl;

#[derive(Copy)]
pub struct Entity {
    x: f32,
    y: f32, 
    width: f32,
    height: f32,
    texture: glium::texture::CompressedSrgbTexture2d,
    vertex_buffer: glium::VertexBuffer<opengl::Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: glium::Program,
    transform: Transform
}

struct Transform {
    translate: Matrix4<f32>,
    rotate: Matrix4<f32>,
    scale: Matrix4<f32>
}

use cgmath::{Matrix4, vec3, Deg};

impl Entity {
    pub fn new(window: &glium::backend::glutin_backend::GlutinFacade, x: f32, y: f32, width: f32, height: f32, shader_vert: &'static str, shader_frag: &'static str, texture_path: &'static str) -> Entity {
        let texture = opengl::load_opengl_texture(window, texture_path);
        let (vertex_buffer, index_buffer) = opengl::get_quad(window);
        let program = opengl::create_program(window, shader_vert, shader_frag);

        let transform = Transform {
            translate: Matrix4::from_translation(vec3(width/2.0+x, height/2.0+y, 0.0)),
            rotate: Matrix4::from_angle_z(Deg(0.0)),
            scale: Matrix4::from_nonuniform_scale(width, height, 0.0)
        };

        Entity {
            x: x,
            y: y,
            width: width, 
            height: height,
            texture: texture,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: program,
            transform: transform
        }
    }
    
    pub fn translate(&mut self, x: f32, y: f32) {
        let translate = Matrix4::from_translation(vec3(x, y, 0.0)) * self.transform.translate;

        self.transform = Transform {
            translate: translate,
            ..self.transform
        };
    }
    
    pub fn rotate(&mut self, deg: f32) {
        let rotate = Matrix4::from_angle_z(Deg(deg)) * self.transform.translate;

        self.transform = Transform {
            rotate: rotate,
            ..self.transform
        };
    }

    pub fn scale(&mut self, scale: f32) {
        let scale = Matrix4::from_scale(scale) * self.transform.scale;

        self.transform = Transform {
            scale: scale,
            ..self.transform
        };
    }

    pub fn draw(&self, target: &mut glium::Frame, view_matrix: [[f32; 4]; 4], projection_matrix: [[f32; 4]; 4]) {
        use glium::Surface;

        let model_matrix: [[f32; 4]; 4] = (self.transform.translate * self.transform.rotate * self.transform.scale).into();

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
            model: model_matrix,
            view: view_matrix,
            projection: projection_matrix,
            tex: &self.texture
        };

        target.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniforms, &params).unwrap();
    }
}