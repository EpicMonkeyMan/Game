#[macro_use]
extern crate glium;
extern crate image;

fn main() {
    use glium::{DisplayBuild, Surface};
    use std::io::Cursor;

    let window = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_multisampling(8u16)
        .with_title("David er en pen person")
        .with_fullscreen(glium::glutin::get_primary_monitor())
        .build_glium()
        .unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../textures/david.png")[..]), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let texture = glium::texture::CompressedSrgbTexture2d::new(&window, image).unwrap();

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
            tex_coords: [f32; 2]
        }

        implement_vertex!(Vertex, position, color, tex_coords);

        glium::VertexBuffer::new(&window, 
            &[
                Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [-0.5,  0.5], color: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0] },
                Vertex { position: [ 0.5,  0.5], color: [0.0, 0.0, 1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [ 0.5, -0.5], color: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0] }
            ]
        ).unwrap()
    };

    let index_buffer = glium::IndexBuffer::new(&window, glium::index::PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();

    let mut vshader_src = String::new();
    let mut fshader_src = String::new();
    unsafe {
        use std::io::{Read, BufReader};
        use std::fs::File;

        let vsf = File::open("shaders/sprite.vert").unwrap();
        let fsf = File::open("shaders/sprite.frag").unwrap();
        let mut vsr = BufReader::new(vsf);
        let mut fsr = BufReader::new(fsf);
        vsr.read_to_end(&mut vshader_src.as_mut_vec()).unwrap();
        fsr.read_to_end(&mut fshader_src.as_mut_vec()).unwrap();
    }

    let program = glium::Program::from_source(&window, vshader_src.as_str(), fshader_src.as_str(), None).unwrap();

    let model_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];
    let view_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];
    let projection_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];

    'gameloop: loop {
        let uniforms = uniform! {
            model: model_matrix,
            view: view_matrix,
            projection: projection_matrix,
            tex: &texture
        };

        let mut target = window.draw();
        target.clear_color(0.2, 0.2, 0.2, 1.0); 
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for e in window.poll_events() {
            use glium::glutin::Event;

            match e {
                Event::Closed => break 'gameloop,
                _ => (),
            }
        }
    }
}
