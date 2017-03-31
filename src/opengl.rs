extern crate glium;
extern crate image;

use std::io::Cursor;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub fn create_program(window: &glium::backend::glutin_backend::GlutinFacade, shader_vert: &'static str, shader_frag: &'static str) -> glium::Program {
    //LOAD SHADERS
    let mut vshader_src = String::new();
    let mut fshader_src = String::new();
    unsafe {
        use std::io::{Read, BufReader};
        use std::fs::File;

        let vsf = File::open(shader_vert).unwrap();
        let fsf = File::open(shader_frag).unwrap();
        let mut vsr = BufReader::new(vsf);
        let mut fsr = BufReader::new(fsf);
        vsr.read_to_end(&mut vshader_src.as_mut_vec()).unwrap();
        fsr.read_to_end(&mut fshader_src.as_mut_vec()).unwrap();
    }

    glium::Program::from_source(window, vshader_src.as_str(), fshader_src.as_str(), None).unwrap()
}

pub fn load_opengl_texture(window: &glium::backend::glutin_backend::GlutinFacade, path_str: &'static str) -> glium::texture::CompressedSrgbTexture2d {
    //LOAD FILE
    let mut path_src = String::new();
    unsafe {
        use std::io::{Read, BufReader};
        use std::fs::File;

        let path_f = File::open(path_str).unwrap();
        let mut path_r = BufReader::new(path_f);
        path_r.read_to_end(&mut path_src.as_mut_vec()).unwrap();
    }
    //LOAD OPENGL TEXTURE
    let image = image::load(Cursor::new(path_src.as_bytes()), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    glium::texture::CompressedSrgbTexture2d::new(window, image).unwrap()
}

pub fn get_quad(window: &glium::backend::glutin_backend::GlutinFacade) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
    //SET VERTEX BUFFER
    let vertex_buffer = {
        implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(window, 
            &[
                Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
                Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 1.0] },
                Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
                Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] }
            ]
        ).unwrap()
    };

    //SET INDEX BUFFER
    let index_buffer = glium::IndexBuffer::new(window, glium::index::PrimitiveType::TriangleStrip, &[1u16, 2, 0, 3]).unwrap();

    (vertex_buffer, index_buffer)
}