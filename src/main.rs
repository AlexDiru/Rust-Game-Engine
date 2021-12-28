mod shaders;
mod geometry;

#[macro_use]
extern crate glium;
extern crate image;
extern crate glutin;

use std::io::Cursor;
use glutin::event::ElementState;
use glutin::event::VirtualKeyCode::P;
use glutin::event::WindowEvent::KeyboardInput;
use crate::ElementState::Pressed;
use crate::geometry::create_quad;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24).with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let shape = create_quad(&display);

    let image = image::load(Cursor::new(&include_bytes!("../assets/tuto-14-diffuse.jpeg")),
                            image::ImageFormat::Jpeg).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../assets/tuto-14-normal.png")),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let normal_map = glium::texture::Texture2d::new(&display, image).unwrap();



    let program = glium::Program::from_source(&display, shaders::vertex_shader_src, shaders::fragment_shader_src,
                                              None).unwrap();

    let mut model = nalgebra_glm::Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0f32
    );

    let mut position = nalgebra_glm::Vec3::new(0.5, 0.2, -3.0);

    let mut movingForward = false;
    let mut movingBackward = false;
    let mut movingLeft = false;
    let mut movingRight = false;
    let mut movingUp = false;
    let mut movingDown = false;
    let mut wireframe_mode = false;

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    match input.scancode {
                        13 => {
                            // w
                            movingForward = input.state == Pressed;
                        },
                        0 => {
                            // a
                            movingLeft = input.state == Pressed;
                        },
                        1 => {
                            // s
                            movingBackward = input.state == Pressed;
                        },
                        2 => {
                            // d
                            movingRight = input.state == Pressed;
                        }
                        40 => {
                            // k
                            if input.state == Pressed {
                                wireframe_mode = !wireframe_mode;
                            }
                        }
                        12 => {
                            // q
                            movingDown = input.state == Pressed;
                        }
                        14 => {
                            // e
                            movingUp = input.state == Pressed;
                        }
                        _ => {
                            println!("scancode {}", input.scancode);
                        }
                    }
                    return;
                },
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        if movingForward {
            position.z = position.z + 0.1;
        }

        if movingBackward {
            position.z = position.z - 0.1;
        

        if movingLeft {
            position.x = position.x - 0.1;
        }

        if movingRight {
            position.x = position.x + 0.1;
        }

        if movingDown {
            position.y = position.y - 0.1;
        }

        if movingUp {
            position.y = position.y + 0.1;
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let view = view_matrix(&position, &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]);

        let (width, height) = target.get_dimensions();
        let mut glm_perspective = nalgebra_glm::perspective_lh(
            width as f32 / height as f32,
            (3.141592 / 3.0) as f32,
            0.1,
            1024.0
        );

        //model = nalgebra_glm::translate(&model, &nalgebra_glm::vec3(0.01, 0.01, 0.01f32));
        model = nalgebra_glm::rotate(&model, 0.01, &nalgebra_glm::vec3(0.2, 1.0, 0.2f32).normalize());

        let light = nalgebra_glm::vec3(1.4, 0.4, 0.7f32);

        let polygon_mode = if wireframe_mode { glium::draw_parameters::PolygonMode::Line } else { glium::draw_parameters::PolygonMode::Fill };

        let params = glium::DrawParameters {
            polygon_mode,
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        target.draw(&shape, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
                    &uniform! {
                        model:  model.data.0,
                        view: view,
                        perspective: glm_perspective.data.0,
                        u_light: light.data.0[0],
                        diffuse_tex: &diffuse_texture,
                        normal_tex: &normal_map
                    },
                    &params).unwrap();
        target.finish().unwrap();
    });
}


fn view_matrix(position: &nalgebra_glm::Vec3, direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position.x * s_norm[0] - position.y * s_norm[1] - position.z * s_norm[2],
        -position.x * u[0] - position.y * u[1] - position.z * u[2],
        -position.x * f[0] - position.y * f[1] - position.z * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}