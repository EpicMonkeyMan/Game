#[macro_use]
extern crate glium;

fn main() {
    use glium::{DisplayBuild, Surface};

    let window = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_title("David er en pen person")
        .with_fullscreen(glium::glutin::get_primary_monitor())
        .build_glium()
        .unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.5,  -0.5] };
    let vertex3 = Vertex { position: [ -0.5, 0.5] };
    let vertex4 = Vertex { position: [ 0.5, 0.5] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&window, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

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

    'gameloop: loop {
        for e in window.poll_events() {
            use glium::glutin::Event;

            match e {
                Event::Closed => break 'gameloop,
                _ => (),
            }
        }

        let mut target = window.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    }
}
