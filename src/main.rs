mod shaders;
mod geometry;
mod cube;
mod map;

#[macro_use]
extern crate glium;
extern crate image;
extern crate glutin;

use std::io::Cursor;
use glutin::event::ElementState;
use glutin::event::VirtualKeyCode::P;
use glutin::event::WindowEvent::KeyboardInput;
use nalgebra_glm::{mat4, Mat4, RealNumber, TMat4};
use crate::ElementState::Pressed;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24).with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../assets/anime.png")),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../assets/anime.png")),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let normal_map = glium::texture::Texture2d::new(&display, image).unwrap();

    let program = glium::Program::from_source(&display,
                                              shaders::vertex_shader_src,
                                              shaders::fragment_shader_custom_light_src,
                                              None).unwrap();

    let mut cubes = [
        cube::Cube::new(&display),
        cube::Cube::new(&display),
        cube::Cube::new(&display),
        cube::Cube::new(&display),
        cube::Cube::new(&display),
        cube::Cube::new(&display),
        cube::Cube::new(&display),
        cube::Cube::new(&display),
        cube::Cube::new(&display)
    ];

    cubes[0].set_position(-2.0, 0.0, 0.0);
    cubes[2].set_position(2.0, 0.0, 0.0);
    cubes[3].set_position(-2.0, -2.0, 0.0);
    cubes[4].set_position(0.0, -2.0, 0.0);
    cubes[5].set_position(2.0, -2.0, 0.0);
    cubes[6].set_position(-2.0, 2.0, 0.0);
    cubes[7].set_position(0.0, 2.0, 0.0);
    cubes[8].set_position(2.0, 2.0, 0.0);

    let mut position = nalgebra_glm::Vec3::new(0.5, 0.2, -3.0);
    let mut light_position = nalgebra_glm::Vec3::new(0.0, 0.0, -2.0);

    let mut movingForward = false;
    let mut movingBackward = false;
    let mut movingLeft = false;
    let mut movingRight = false;
    let mut movingUp = false;
    let mut movingDown = false;
    let mut wireframe_mode = false;
    let mut upArrowHeld = false;
    let mut leftArrowHeld = false;
    let mut downArrowHeld = false;
    let mut rightArrowHeld = false;

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
                        126 => {
                            // up
                            upArrowHeld = input.state == Pressed;
                        }
                        123 => {
                            //left
                            leftArrowHeld = input.state == Pressed;
                        }
                        125 => {
                            //down
                            downArrowHeld = input.state == Pressed;
                        }
                        124 => {
                            //right
                            rightArrowHeld = input.state == Pressed;
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
        }

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

        if rightArrowHeld {
            light_position.x = light_position.x + 0.1;
        }

        if leftArrowHeld {
            light_position.x = light_position.x - 0.1;
        }

        if upArrowHeld {
            light_position.y = light_position.z + 0.1;
        }

        if downArrowHeld {
            light_position.y = light_position.z - 0.1;
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let view = view_matrix(
            &position, // Camera pos
            &nalgebra_glm::Vec3::new(-0.5, -0.2, 3.0),
            &nalgebra_glm::Vec3::new(0.0, 1.0, 0.0));

        let (width, height) = target.get_dimensions();
        let mut glm_perspective = nalgebra_glm::perspective_lh(
            width as f32 / height as f32,
            (3.141592 / 3.0) as f32,
            0.1,
            1024.0
        );

        //model = nalgebra_glm::translate(&model, &nalgebra_glm::vec3(0.01, 0.01, 0.01f32));
        for mut cube in &mut cubes {
            let pos = cube.get_position();
            //cube.rotate(0.01, pos[0], pos[1], pos[2]);
        }

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

        for mut cube in &cubes {
            let v_buffer = cube.create_vertex_buffer(&display);
            let light = (cube.get_position_vec() + light_position).data.0[0];

            target.draw(&v_buffer,
                        glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                        &program,
                        &uniform! {
                            model: cube.get_mat(),
                            view: view.data.0,
                            perspective: mat_to_arr(glm_perspective),
                            u_light: light,
                            diffuse_tex: &diffuse_texture,
                            normal_tex: &diffuse_texture,
                            intensity: 0.8f32,
                    },
                        &params).unwrap();
        }

        target.finish().unwrap();
    });
}


fn view_matrix(position: &nalgebra_glm::Vec3, direction: &nalgebra_glm::Vec3, up: &nalgebra_glm::Vec3) -> Mat4 {

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

    nalgebra_glm::mat4(
        s_norm[0], s_norm[1], s_norm[2], p[0],
        u[0], u[1], u[2], p[1],
        f[0], f[1], f[2], p[2],
        0.0, 0.0, 0.0, 1.0,
    )
}

fn mat_to_arr<T>(mat4: nalgebra_glm::TMat4<T>) -> [[T; 4]; 4] where T: RealNumber {
    [
        [mat4[(0,0)], mat4[(1, 0)], mat4[(2, 0)], mat4[(3, 0)] ],
        [mat4[(0,1)], mat4[(1, 1)], mat4[(2, 1)], mat4[(3, 1)] ],
        [mat4[(0,2)], mat4[(1, 2)], mat4[(2, 2)], mat4[(3, 2)] ],
        [mat4[(0,3)], mat4[(1, 3)], mat4[(2, 3)], mat4[(3, 3)] ],
    ]
}