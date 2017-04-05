#[macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;

use glium::{DisplayBuild, Surface};
use glium::glutin::VirtualKeyCode;

mod opengl;
mod entity;
mod matrix;
mod networking;

fn main() {
    //CREATE WINDOW
    let window = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(800, 600)
        .with_multisampling(8u16)
        .with_title("David er en pen person")
        .with_depth_buffer(24)
        //.with_fullscreen(glium::glutin::get_primary_monitor())
        .build_glium()
        .unwrap();
    
    /*
    //NETWORKING
    let mut network = networking::Network::new();
    network.server();
    */
    
    //SET WIDTH AND HEIGHT
    let (window_width, window_height) = window.get_framebuffer_dimensions();

    //ENTITY
    let background = entity::Entity::new(&window, 0.0, 0.0, 0.0, window_width as f32, window_height as f32, 0.0,
                "shaders/sprite.vert", "shaders/sprite.frag", "textures/background.png");
    let david = entity::Entity::new(&window, 0.0, 0.0, 1.0, 150.0, 150.0, 0.0,
                "shaders/sprite.vert", "shaders/sprite.frag", "textures/david.png");
    let dennis = entity::Entity::new(&window, 0.0, 0.0, 2.0, 150.0, 150.0, 0.0,
                "shaders/sprite.vert", "shaders/sprite.frag", "textures/dennis.png");
  
    let mut entities = vec![background, david, dennis];

    //SET TEMP
    let mut key_states = std::collections::HashMap::new();

    let mut view_matrix = matrix::Matrix::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.0);

    'gameloop: loop {
        //UPDATE WIDTH AND HEIGHT
        let (window_width, window_height) = window.get_framebuffer_dimensions();
        
        entities[0].model_matrix.set_nonuniform_scale(window_width as f32, window_height as f32);
        let (x, y) = (entities[0].model_matrix.width/2.0, entities[0].model_matrix.height/2.0);
        entities[0].model_matrix.set_translation(x, y, 0.0);

        //KEY
        if key_states.get(&VirtualKeyCode::Left) == Some(&true) {view_matrix.translate(5.0, 0.0, 0.0);}
        if key_states.get(&VirtualKeyCode::Right) == Some(&true) {view_matrix.translate(-5.0, 0.0, 0.0);}
        if key_states.get(&VirtualKeyCode::Up) == Some(&true) {view_matrix.translate(0.0, 5.0, 0.0);}
        if key_states.get(&VirtualKeyCode::Down) == Some(&true) {view_matrix.translate(0.0, -5.0, 0.0);}

        
        if view_matrix.x + entities[1].model_matrix.x < window_width as f32 /3.0 {
            view_matrix.translate(5.0, 0.0, 0.0);
        } else if view_matrix.x + entities[1].model_matrix.x > window_width as f32 - window_width as f32 /3.0 {
            view_matrix.translate(-5.0, 0.0, 0.0);
        }
        if view_matrix.y + entities[1].model_matrix.y < window_height as f32 /3.0 {
            view_matrix.translate(0.0, 5.0, 0.0);
        } else if view_matrix.y + entities[1].model_matrix.y > window_height as f32 - window_height as f32 /3.0 {
            view_matrix.translate(0.0, -5.0, 0.0);
        }

        if key_states.get(&VirtualKeyCode::D) == Some(&true) {entities[1].model_matrix.translate(5.0, 0.0, 0.0);}
        if key_states.get(&VirtualKeyCode::A) == Some(&true) {entities[1].model_matrix.translate(-5.0, 0.0, 0.0);}
        if key_states.get(&VirtualKeyCode::W) == Some(&true) {entities[1].model_matrix.translate(0.0, -5.0, 0.0);}
        if key_states.get(&VirtualKeyCode::S) == Some(&true) {entities[1].model_matrix.translate(0.0, 5.0, 0.0);}
        if key_states.get(&VirtualKeyCode::E) == Some(&true) {entities[1].model_matrix.rotate(10.0);}
        if key_states.get(&VirtualKeyCode::Q) == Some(&true) {entities[1].model_matrix.rotate(-10.0);}
        if key_states.get(&VirtualKeyCode::T) == Some(&true) {entities[1].model_matrix.translate(0.0, 0.0, 1.0);}
        if key_states.get(&VirtualKeyCode::Y) == Some(&true) {entities[1].model_matrix.translate(0.0, 0.0, -1.0);}
        
        /*
        {
            let xlock = network.x_ratio.lock().unwrap();
            let ylock = network.y_ratio.lock().unwrap();

            entities[2].model_matrix.translate(*xlock, *ylock, 0.0);
        }
        */

        println!("SCRN_W: {} SCRN_H: {} X: {} Y: {} Z: {} Width {} Height: {} Rotation: {}",
            window_width, window_height, entities[1].model_matrix.x, entities[1].model_matrix.y, entities[1].model_matrix.z,
            entities[1].model_matrix.width, entities[1].model_matrix.height, entities[1].model_matrix.rotation);        

        //PROJECTION MATRIX
        //let projection_matrix: [[f32; 4]; 4] = cgmath::perspective(cgmath::Deg(90f32), (window_width/window_height) as f32, 0.1f32, 1000f32).into();
        let projection_matrix: [[f32; 4]; 4] = cgmath::ortho(0.0f32, window_width as f32, window_height as f32, 0.0f32, -100.0f32, 1.0f32).into();

        //DRAW
        let mut target = window.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.2, 1.0), 1.0);
        
        for e in entities.iter() {
            e.draw(&mut target, view_matrix.get_matrix(), projection_matrix);
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
