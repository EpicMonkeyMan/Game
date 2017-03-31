#[macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;

use glium::{DisplayBuild, Surface};
use cgmath::{vec3, Matrix4};
use glium::glutin::VirtualKeyCode;

mod opengl;
mod entity;

fn main() {
    //CREATE WINDOW
    let window = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(800, 600)
        .with_multisampling(8u16)
        .with_title("David er en pen person")
        //.with_fullscreen(glium::glutin::get_primary_monitor())
        .build_glium()
        .unwrap();
    
    //SET WIDTH AND HEIGHT
    let (window_width, window_height) = window.get_framebuffer_dimensions();

    //ENTITY
    let background = entity::Entity::new(&window, 0.0, 0.0, window_width as f32, window_height as f32,
                "shaders/sprite.vert", "shaders/sprite.frag", "textures/background.png");
    let david = entity::Entity::new(&window, 0.0, 0.0, 150.0, 150.0,
                "shaders/sprite.vert", "shaders/sprite.frag", "textures/david.png");
  
    let mut entities = vec![background, david];

    //SET TEMP
    let mut key_states = std::collections::HashMap::new();
    let (mut x, mut y) = (0.0, 0.0);

    'gameloop: loop {
        //UPDATE WIDTH AND HEIGHT
        let (window_width, window_height) = window.get_framebuffer_dimensions();
        entities[0].set_nonuniform_scale(window_width as f32, window_height as f32);
        entities[0].set_translation(0.0, 0.0);

        //KEY
        if key_states.get(&VirtualKeyCode::Left) == Some(&true) {x-=10.0;}
        if key_states.get(&VirtualKeyCode::Right) == Some(&true) {x+=10.0;}
        if key_states.get(&VirtualKeyCode::Up) == Some(&true) {y-=10.0;}
        if key_states.get(&VirtualKeyCode::Down) == Some(&true) {y+=10.0;}
        if key_states.get(&VirtualKeyCode::D) == Some(&true) {entities[1].translate(5.0, 0.0);}
        if key_states.get(&VirtualKeyCode::A) == Some(&true) {entities[1].translate(-5.0, 0.0);}
        if key_states.get(&VirtualKeyCode::W) == Some(&true) {entities[1].translate(0.0, -5.0);}
        if key_states.get(&VirtualKeyCode::S) == Some(&true) {entities[1].translate(0.0, 5.0);}
        if key_states.get(&VirtualKeyCode::E) == Some(&true) {entities[1].rotate(10.0);}
        if key_states.get(&VirtualKeyCode::Q) == Some(&true) {entities[1].rotate(-10.0);}

        println!("SCRN_W: {} SCRN_H: {} X: {} Y: {} Width {} Height: {} Rotation: {}", window_width, window_height, entities[1].x, entities[1].y, entities[1].width, entities[1].height, entities[1].rotation);

        //VIEW MATRIX
        let view_matrix: [[f32; 4]; 4] = Matrix4::from_translation(vec3(-x, -y, 0.0)).into();

        //PROJECTION MATRIX
        //let projection_matrix: [[f32; 4]; 4] = cgmath::perspective(Deg(90f32), (window_width/window_height) as f32, 0.1f32, 100f32).into();
        let projection_matrix: [[f32; 4]; 4] = cgmath::ortho(0.0f32, window_width as f32, window_height as f32, 0.0f32, -1.0f32, 1.0f32).into();

        //DRAW
        let mut target = window.draw();
        target.clear_color(0.2, 0.2, 0.2, 1.0);
        
        for e in entities.iter() {
            e.draw(&mut target, view_matrix, projection_matrix);
        }
        target.finish().unwrap();

        //QUIT
        if key_states.get(&VirtualKeyCode::Escape) == Some(&true) { break 'gameloop; }

        //REBUILD PROGRAM
        //if key_states.get(&VirtualKeyCode::R) == Some(&true) { program = opengl::create_program(&window, "shaders/sprite.vert", "shaders/sprite.frag"); }

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
