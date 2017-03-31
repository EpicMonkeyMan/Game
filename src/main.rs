#[macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;

use glium::{DisplayBuild, Surface};
use cgmath::{Deg, vec3, Matrix4};
use glium::glutin::VirtualKeyCode;

mod opengl;
mod entity;

fn main() {
    //CREATE WINDOW
    let window = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(800, 600)
        .with_multisampling(8u16)
        .with_depth_buffer(24)
        .with_title("David er en pen person")
        //.with_fullscreen(glium::glutin::get_primary_monitor())
        .build_glium()
        .unwrap();

    //SET TEXTURE
    let texture = opengl::load_opengl_texture(&window, "textures/david.png");

    //SET BUFFERS
    let (vertex_buffer, index_buffer) = opengl::get_quad(&window);

    //SET PROGRAM
    let mut program = opengl::create_program(&window, "shaders/sprite.vert", "shaders/sprite.frag");

    //SET TEMP
    let mut key_states = std::collections::HashMap::new();
    let (mut x, mut y) = (0.0, 0.0);
    let mut deg = 0.0;

    let david = entity::Entity::new(&window, 0.0, 0.0, 300.0, 300.0,
                "shaders/sprite.vert", "shaders/sprite.frag", "textures/david.png");
    let david2 = entity::Entity::new(&window, 600.0, 600.0, 300.0, 300.0,
                "shaders/sprite.vert", "shaders/sprite.frag", "textures/david.png");

    'gameloop: loop {
        //SET WIDTH / HEIGHT
        let (window_width, window_height) = window.get_framebuffer_dimensions();

        //MODEL MATRIX
        if key_states.get(&VirtualKeyCode::Left) == Some(&true) {x-=10.0; deg-=10.0}
        if key_states.get(&VirtualKeyCode::Right) == Some(&true) {x+=10.0; deg+=10.0}
        if key_states.get(&VirtualKeyCode::Up) == Some(&true) {y-=10.0;}
        if key_states.get(&VirtualKeyCode::Down) == Some(&true) {y+=10.0;}
        if key_states.get(&VirtualKeyCode::D) == Some(&true) {deg+=10.0;}
        if key_states.get(&VirtualKeyCode::A) == Some(&true) {deg-=10.0;}
        let scale = 150.0;
        let translate = Matrix4::from_translation(vec3(scale/2.0+x, scale/2.0+y, 0.0));
        let rotate = Matrix4::from_angle_z(Deg(deg));
        let scale = Matrix4::from_scale(scale);
        let model_matrix: [[f32; 4]; 4] = (translate * rotate * scale).into();

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
        target.clear_color_and_depth((0.2, 0.2, 0.2, 1.0), 1.0); 
        //target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params).unwrap();
        david.draw(&mut target, view_matrix, projection_matrix);
        david2.draw(&mut target, view_matrix, projection_matrix);
        target.finish().unwrap();

        //QUIT
        if key_states.get(&VirtualKeyCode::Escape) == Some(&true) { break 'gameloop; }

        //REBUILD PROGRAM
        if key_states.get(&VirtualKeyCode::R) == Some(&true) { program = opengl::create_program(&window, "shaders/sprite.vert", "shaders/sprite.frag"); }

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
