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
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&window, &shape).unwrap();   
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }"#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;
    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }"#;

    let program = glium::Program::from_source(&window, vertex_shader_src, fragment_shader_src, None).unwrap();

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
