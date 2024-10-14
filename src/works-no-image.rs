// Uncomment these following global attributes to silence most warnings of "low" interest:

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]

extern crate nalgebra_glm as glm;
use std::{mem, ptr, os::raw::c_void};
use std::thread;
use std::sync::{Mutex, Arc, RwLock};
use std::ffi::CString;

mod shader;
mod util;
mod mesh;
use mesh::Helicopter;
mod scene_graph;
use scene_graph::SceneNode;
mod toolbox;

use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //
//
// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  byte_size_of_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()

// == // Generate your VAO here
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, colors: &Vec<f32>, normals: &Vec<f32>) -> u32 {
    let mut vao: u32 = 0;

    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    // Positions
    let mut position_vbo: u32 = 0;
    gl::GenBuffers(1, &mut position_vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, position_vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertices),
        pointer_to_array(vertices),
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
    gl::EnableVertexAttribArray(0);

    // Colors
    let mut color_vbo: u32 = 0;
    gl::GenBuffers(1, &mut color_vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, color_vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(colors),
        pointer_to_array(colors),
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());
    gl::EnableVertexAttribArray(1);

    // Normals
    let mut normal_vbo: u32 = 0;
    gl::GenBuffers(1, &mut normal_vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, normal_vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(normals),
        pointer_to_array(normals),
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
    gl::EnableVertexAttribArray(2);

    // Indices
    let mut ebo: u32 = 0;
    gl::GenBuffers(1, &mut ebo);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        pointer_to_array(indices),
        gl::STATIC_DRAW,
    );

    // Unbind VAO (optional but good practice)
    gl::BindVertexArray(0);

    vao
}

fn draw_scene(
    node: &SceneNode,
    transformation_so_far: &glm::Mat4,
    view_projection_matrix: &glm::Mat4,
    shader_program: u32,
) {
    // Local transformation matrix
    let mut local_transform = glm::translate(&glm::identity(), &node.position);
    local_transform = glm::translate(&local_transform, &node.reference_point);
    local_transform = glm::rotate_x(&local_transform, node.rotation.x);
    local_transform = glm::rotate_y(&local_transform, node.rotation.y);
    local_transform = glm::rotate_z(&local_transform, node.rotation.z);
    local_transform = glm::translate(&local_transform, &(-node.reference_point));
    local_transform = glm::scale(&local_transform, &node.scale);

    // Current transformation
    let current_transformation = transformation_so_far * local_transform;

    // Draw the node if it has a VAO
    if node.vao_id != 0 && node.index_count > 0 {
        let mvp_matrix = view_projection_matrix * current_transformation;
        let model_matrix = current_transformation;

        unsafe {
            let mvp_loc =
                gl::GetUniformLocation(shader_program, CString::new("MVP").unwrap().as_ptr());
            gl::UniformMatrix4fv(mvp_loc, 1, gl::FALSE, mvp_matrix.as_ptr());

            let model_loc =
                gl::GetUniformLocation(shader_program, CString::new("model_matrix").unwrap().as_ptr());
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model_matrix.as_ptr());

            gl::BindVertexArray(node.vao_id);
            gl::DrawElements(gl::TRIANGLES, node.index_count, gl::UNSIGNED_INT, ptr::null());
        }
    }

    // Recursively draw child nodes
    for &child_ptr in &node.children {
        let child_ref: &SceneNode = unsafe { &*child_ptr };
        draw_scene(
            child_ref,
            &current_transformation,
            view_projection_matrix,
            shader_program,
        );
    }
}


fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();

    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    let window_size = Arc::clone(&arc_window_size);

    let render_thread = thread::spawn(move || {
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        let simple_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
        };

        let mut camera_position = glm::vec3(0.0, 50.0, 200.0);

        // Load the terrain mesh
        let terrain_mesh = mesh::Terrain::load("./resources/lunarsurface.obj");

        // Create the VAO
        let terrain_vao = unsafe {
            create_vao(
                &terrain_mesh.vertices,
                &terrain_mesh.indices,
                &terrain_mesh.colors,
                &terrain_mesh.normals,
            )
        };

        let helicopter = Helicopter::load("./resources/helicopter.obj");
        // Create VAOs for each helicopter part
        let helicopter_body_vao = unsafe {
            create_vao(
                &helicopter.body.vertices,
                &helicopter.body.indices,
                &helicopter.body.colors,
                &helicopter.body.normals,
            )
        };

        let helicopter_door_vao = unsafe {
            create_vao(
                &helicopter.door.vertices,
                &helicopter.door.indices,
                &helicopter.door.colors,
                &helicopter.door.normals,
            )
        };

        let helicopter_main_rotor_vao = unsafe {
            create_vao(
                &helicopter.main_rotor.vertices,
                &helicopter.main_rotor.indices,
                &helicopter.main_rotor.colors,
                &helicopter.main_rotor.normals,
            )
        };

        let helicopter_tail_rotor_vao = unsafe {
            create_vao(
                &helicopter.tail_rotor.vertices,
                &helicopter.tail_rotor.indices,
                &helicopter.tail_rotor.colors,
                &helicopter.tail_rotor.normals,
            )
        };

        // Create the root node of the scene
        let mut root_node = SceneNode::new();

        // Create the terrain node
        let mut terrain_node = SceneNode::from_vao(terrain_vao, terrain_mesh.index_count);
        terrain_node.reference_point = glm::vec3(0.0, 0.0, 0.0);

        // Create helicopter nodes
        let mut helicopter_body_node = SceneNode::from_vao(helicopter_body_vao, helicopter.body.index_count);
        let mut helicopter_door_node = SceneNode::from_vao(helicopter_door_vao, helicopter.door.index_count);
        let mut helicopter_main_rotor_node = SceneNode::from_vao(helicopter_main_rotor_vao, helicopter.main_rotor.index_count);
        let mut helicopter_tail_rotor_node = SceneNode::from_vao(helicopter_tail_rotor_vao, helicopter.tail_rotor.index_count);

        // Set reference points
        helicopter_body_node.reference_point = glm::vec3(0.0, 0.0, 0.0);
        helicopter_main_rotor_node.reference_point = glm::vec3(0.0, 0.0, 0.0);
        helicopter_tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);

        // Build the scene graph
        helicopter_body_node.add_child(&helicopter_door_node);
        helicopter_body_node.add_child(&helicopter_main_rotor_node);
        helicopter_body_node.add_child(&helicopter_tail_rotor_node);

        terrain_node.add_child(&helicopter_body_node);
        root_node.add_child(&terrain_node);

        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;

        let rotor_speed = 10.0;
        let mut helicopter_body_position_y = 50.0;

        loop {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Window was resized to {}x{}", new_size.0, new_size.1);
                    unsafe { gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32); }
                }
            }
            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        VirtualKeyCode::A => {
                            camera_position.x -= 10.0 * delta_time;
                        }
                        VirtualKeyCode::D => {
                            camera_position.x += 10.0 * delta_time;
                        }
                        VirtualKeyCode::W => {
                            camera_position.z -= 10.0 * delta_time;
                        }
                        VirtualKeyCode::S => {
                            camera_position.z += 10.0 * delta_time;
                        }
                        VirtualKeyCode::Space => {
                            camera_position.y += 10.0 * delta_time;
                        }
                        VirtualKeyCode::LShift => {
                            camera_position.y -= 10.0 * delta_time;
                        }
                        _ => {}
                    }
                }
            }
            if let Ok(mut delta) = mouse_delta.lock() {
                *delta = (0.0, 0.0); // reset when done
            }

            // Update rotor rotations
            helicopter_main_rotor_node.rotation.y = elapsed * rotor_speed;
            helicopter_tail_rotor_node.rotation.x = elapsed * rotor_speed;

            // Update helicopter position and rotation
            let heading = toolbox::simple_heading_animation(elapsed);
            helicopter_body_node.position.x = heading.x;
            helicopter_body_node.position.z = heading.z;
            helicopter_body_node.position.y = helicopter_body_position_y;
            helicopter_body_node.rotation.x = heading.roll;
            helicopter_body_node.rotation.y = heading.pitch;
            helicopter_body_node.rotation.z = heading.yaw;

            let projection_matrix = glm::perspective(window_aspect_ratio, 45.0_f32.to_radians(), 0.1, 1000.0);
            let view_matrix = glm::look_at(
                &camera_position,
                &(camera_position + glm::vec3(0.0, 0.0, -1.0)),
                &glm::vec3(0.0, 1.0, 0.0),
            );

            let view_projection_matrix = projection_matrix * view_matrix;

            // Clear the screen
            unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            // Activate the shader program
            unsafe {
                simple_shader.activate();
            }

            // Draw the scene graph
            draw_scene(
                &root_node,
                &glm::identity(),
                &view_projection_matrix,
                simple_shader.program_id,
            );

            context.swap_buffers().unwrap();
        }
    });

    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                println!("New window size received: {}x{}", physical_size.width, physical_size.height);
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                match keycode {
                    Escape => { *control_flow = ControlFlow::Exit; }
                    Q      => { *control_flow = ControlFlow::Exit; }
                    _      => { }
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => { }
        }
    });
}

