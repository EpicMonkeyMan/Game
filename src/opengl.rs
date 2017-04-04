extern crate glium;
extern crate image;

use std::io::Cursor;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

pub fn load_file(src: &'static str) -> String {
    use std::io::{Read, BufReader};
    use std::fs::File;

    let mut src_buffer = String::new();
    let src_file = File::open(src).unwrap();
    let mut src_bufreader = BufReader::new(src_file);
    unsafe {src_bufreader.read_to_end(src_buffer.as_mut_vec()).unwrap();}

    src_buffer
}

#[inline]
pub fn create_program(window: &glium::backend::glutin_backend::GlutinFacade, shader_vert: &'static str, shader_frag: &'static str) -> glium::Program {
    glium::Program::from_source(window, load_file(shader_vert).as_str(), load_file(shader_frag).as_str(), None).unwrap()
}

pub fn load_opengl_texture(window: &glium::backend::glutin_backend::GlutinFacade, path_str: &'static str) -> glium::texture::CompressedSrgbTexture2d {
    //LOAD OPENGL TEXTURE
    let image = image::load(Cursor::new(load_file(path_str).as_bytes()), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    glium::texture::CompressedSrgbTexture2d::new(window, image).unwrap()
}

pub fn get_quad(window: &glium::backend::glutin_backend::GlutinFacade) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
    //SET VERTEX BUFFER
    let vertex_buffer = {
        implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(window, 
            &[
                Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [-0.5,  0.5, 0.0], tex_coords: [0.0, 1.0] },
                Vertex { position: [ 0.5,  0.5, 0.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [ 0.5, -0.5, 0.0], tex_coords: [1.0, 0.0] }
            ]
        ).unwrap()
    };

    //SET INDEX BUFFER
    let index_buffer = glium::IndexBuffer::new(window, glium::index::PrimitiveType::TriangleStrip, &[1u16, 2, 0, 3]).unwrap();

    (vertex_buffer, index_buffer)
}