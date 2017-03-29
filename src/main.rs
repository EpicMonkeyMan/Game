#[macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;

mod entity;

fn main() {
    use glium::{DisplayBuild, Surface};
    use std::io::Cursor;
    use cgmath::{Deg, vec3, Matrix4};
    use glium::glutin::VirtualKeyCode;

    //CREATE WINDOW
    let window = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(800, 600)
        .with_multisampling(8u16)
        .with_title("David er en pen person")
        //.with_fullscreen(glium::glutin::get_primary_monitor())
        .build_glium()
        .unwrap();

    //LOAD OPENGL TEXTURE
    let image = image::load(Cursor::new(&include_bytes!("../textures/david.png")[..]), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let texture = glium::texture::CompressedSrgbTexture2d::new(&window, image).unwrap();

    //SET VERTEX BUFFER
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

    //let ent = entity::Model::new(entity::Entity{x: 0.0, y: 0.0, width: 50.0, height: 50.0, texture_path: "david.png"});

    //SET INDEX BUFFER
    let index_buffer = glium::IndexBuffer::new(&window, glium::index::PrimitiveType::TriangleStrip, &[1u16, 2, 0, 3]).unwrap();

    //LOAD SHADERS
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

    //SET PROGRAM
    let program = glium::Program::from_source(&window, vshader_src.as_str(), fshader_src.as_str(), None).unwrap();

    let mut key_states = std::collections::HashMap::new();
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    let mut deg: f32 = 0.0;
    'gameloop: loop {
        deg+=1.0;

        //SET WIDTH / HEIGHT
        let (window_width, window_height) = window.get_framebuffer_dimensions();

        //MODEL MATRIX
        if key_states.get(&VirtualKeyCode::Left) == Some(&true) {x-=10.0;}
        if key_states.get(&VirtualKeyCode::Right) == Some(&true) {x+=10.0;}
        if key_states.get(&VirtualKeyCode::Up) == Some(&true) {y-=10.0;}
        if key_states.get(&VirtualKeyCode::Down) == Some(&true) {y+=10.0;}
        let translate = Matrix4::from_translation(vec3(50.0+x, 50.0+y, 0.0));
        let rotate = Matrix4::from_angle_z(Deg(deg));
        let scale = Matrix4::from_scale(300.0);
        let model_matrix: [[f32; 4]; 4] = (translate * rotate * scale).into();
        println!("{}", x);

        //VIEW MATRIX
        let view_matrix: [[f32; 4]; 4] = Matrix4::from_translation(vec3(0.0, 0.0, 0.0)).into();

        //PROJECTION MATRIX
        //let projection_matrix: [[f32; 4]; 4] = cgmath::perspective(Deg(140f32), (window_width/window_height) as f32, 0.1f32, 100f32).into();
        let projection_matrix: [[f32; 4]; 4] = cgmath::ortho(0.0f32, window_width as f32, window_height as f32, 0.0f32, -1.0f32, 1.0f32).into();

        //SET UNIFORMS
        let uniforms = uniform! {
            model: model_matrix,
            view: view_matrix,
            projection: projection_matrix,
            tex: &texture
        };

        //DRAW
        let mut target = window.draw();
        target.clear_color(0.2, 0.2, 0.2, 1.0); 
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        //QUIT
        if key_states.get(&VirtualKeyCode::Escape) == Some(&true) { break 'gameloop; }

        //EVENTS
        for e in window.poll_events() {
            use glium::glutin::{Event, ElementState};

            match e {
                Event::Closed => break 'gameloop,
                Event::KeyboardInput(ElementState::Pressed, _, Some(key)) => {
                    key_states.insert(key, true);
                },
                Event::KeyboardInput(ElementState::Released, _, Some(key)) => {
                    key_states.insert(key, false);
                },
                _ => (),
            }
        }
    }
}
