// Uncomment these following global attributes to silence most warnings of "low" interest:

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]

extern crate nalgebra_glm as glm;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::{mem, os::raw::c_void, ptr};

mod shader;
mod util;

use glutin::event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use glutin::event_loop::ControlFlow;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;



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



// TASK 1 a)
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, colors: &Vec<f32>) -> u32 {
    // defining variables
    let mut vao = 0;
    let mut vbo = 0;
    let mut ibo = 0;
     let mut cbo = 0;
    // vertex array objects
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    // vertex buffer objects
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertices),
        pointer_to_array(vertices),
        gl::STATIC_DRAW,
    );

    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        0,
        offset::<f32>(0),
    );
    gl::EnableVertexAttribArray(0);

    // color buffer
    gl::GenBuffers(1, &mut cbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, cbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(colors),
        pointer_to_array(colors),
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        0,
        offset::<f32>(0),
    );
    gl::EnableVertexAttribArray(1);

    //index buffer
    gl::GenBuffers(1, &mut ibo);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);

    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        pointer_to_array(indices),
        gl::STATIC_DRAW,
    );

    // Unbind the VAO and VBO to avoid accidental modification elsewhere
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);

    vao
}


fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(
            INITIAL_SCREEN_W,
            INITIAL_SCREEN_H,
        ));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CCW);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!(
                "{}: {}",
                util::get_gl_string(gl::VENDOR),
                util::get_gl_string(gl::RENDERER)
            );
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!(
                "GLSL\t: {}",
                util::get_gl_string(gl::SHADING_LANGUAGE_VERSION)
            );
        }



// 3 triangles. All overlap near (0,0). Each triangle's 3 vertices have the same z value.
// z_far > z_mid > z_near added f32 to use memory efficiently
let z_far  =  0.30_f32;  // draw first, blue
let z_middle  =  0.00_f32;  // draw second, green
let z_near = -0.30_f32;  // draw last, red

//change to original order
let vertices: Vec<f32> = vec![
    // Triangle A now at NEAR depth (was far) - Red triangle closest
    -0.6, -0.2, z_far,
     0.6, -0.2, z_far,
     0.0,  0.6, z_far,

    // Triangle B now at MIDDLE depth
     -0.6,0.2, z_middle,
     0.6,0.2, z_middle,
     0.0,-0.6, z_middle,

    // Triangle C now at FAR depth (was middle) - Blue triangle farthest
    -0.2, -0.6, z_near,
    -0.2,  0.6, z_near,
     0.6,  0.0, z_near,
];

// let vertices: Vec<f32> = vec![
//     // Triangle A now at NEAR depth (was far) - Red triangle closest
//     -0.6, -0.2, z_near,
//      0.6, -0.2, z_near,
//      0.0,  0.6, z_near,

//     // Triangle B now at MIDDLE depth
//      -0.6,0.2, z_middle,
//      0.6,0.2, z_middle,
//      0.0,-0.6, z_middle,

//     // Triangle C now at FAR depth (was middle) - Blue triangle farthest
//     -0.2, -0.6, z_far,
//     -0.2,  0.6, z_far,
//      0.6,  0.0, z_far,
// ];

//Each triangle has SAME color per vertex, DIFFERENT colors between triangles  
let colors: Vec<f32> = vec![
    // Triangle A: ALL RED vertices (now at NEAR depth, z=-0.30)
    1.0, 0.0, 0.0,  1.0, 0.0, 0.0,  1.0, 0.0, 0.0,
    
    // Triangle B: ALL GREEN vertices (now at FAR depth, z=0.30)
    0.0, 1.0, 0.0,  0.0, 1.0, 0.0,  0.0, 1.0, 0.0,
    
    // Triangle C: ALL BLUE vertices (now at MIDDLE depth, z=0.00)
    0.0, 0.0, 1.0,  0.0, 0.0, 1.0,  0.0, 0.0, 1.0,
];

// CCW indices per triangle
let indices: Vec<u32> = vec![
    0, 1, 2,   // A (far)
    3, 4, 5,   // B (mid)
    6, 7, 8,   // C (near)
];



        let my_vao = unsafe { create_vao(&vertices, &indices, &colors) };
        // TASK 1 b)
        let shader_program = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("shaders/simple.vert")
                .attach_file("shaders/simple.frag")
                .link()
        };
        
unsafe { shader_program.activate(); }
//  need to find the alpha
let alpha: i32 = unsafe {
    let mut current_program: i32 = 0;
    // gets the currently bound program (activated above)
    gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut current_program);
    // find the location of the uniform variable "uAlpha" in the shader
    gl::GetUniformLocation(
        current_program as u32,
        b"uAlpha\0".as_ptr() as *const _
    )
};

// Get uniform location for transformation matrix
let u_transform_matrix: i32 = unsafe {
    let mut current_program: i32 = 0;
    gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut current_program);
    gl::GetUniformLocation(
        current_program as u32,
        b"uTransformMatrix\0".as_ptr() as *const _
    )
};
        

        // Used to demonstrate keyboard handling for exercise 2.
        let mut _arbitrary_number = 0.0; // feel free to remove

        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;
        loop {
            // Compute time passed since the previous frame and since the start of the program
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
                    unsafe {
                        gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32);
                    }
                }
            }

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html
                        VirtualKeyCode::A => {
                            _arbitrary_number += delta_time;
                        }
                        VirtualKeyCode::D => {
                            _arbitrary_number -= delta_time;
                        }

                        // default handler:
                        _ => {}
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {
                // == // Optionally access the accumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

            

            unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // == // Issue the necessary gl:: commands to draw your scene here

        

    // cull face is disabled to see all triangles
    gl::Disable(gl::CULL_FACE);

    gl::DepthMask(gl::TRUE);  
    gl::Uniform1f(alpha, 0.4);

    // Create transformation matrix on CPU
    // Mode: which matrix element to modify (0=a, 1=b, 2=c, 3=d, 4=e, 5=f)
    //et current_mode = 5; // 0=a(X-scale), 1=b(X-shear), 2=c(X-translate), 3=d(Y-shear), 4=e(Y-scale), 5=f(Y-translate)
    //let oscillating_value = (elapsed * 2.0).sin(); // Oscillate between -1 and 1
    let perspective_matrix: glm::Mat4 = glm::perspective(
        window_aspect_ratio,  // aspect ratio
        1.2,                  // fovy (field of view in radians) - about 69 degrees
        1.0,                  // near clipping plane
        100.0                 // far clipping plane
    );
    
    let mut additional_transform = glm::Mat4::identity();
    // Create translation matrix to move triangles into negative z range
    // We'll translate to z = -10 to place triangles well within the visible range (-1 to -100)
    let translation_matrix: glm::Mat4 = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 0.0, -10.0));
    // TEMP for the demo:
    
    
    
    // Combine transformations: perspective * translation * additional_transform
    // Note: matrix multiplication is right-to-left, so this applies:
    // 1. additional_transform to vertices first
    // 2. then translation 
    // 3. finally perspective projection
    //let transform_matrix = translation_matrix; // no projection → triangles disappear
    let transform_matrix: glm::Mat4 = perspective_matrix * translation_matrix * additional_transform;
    
    // Pass matrix to shader (activate shader program first)
    shader_program.activate();
    gl::UniformMatrix4fv(u_transform_matrix, 1, gl::FALSE, transform_matrix.as_ptr());

    gl::BindVertexArray(my_vao);

    let isz = size_of::<u32>() as isize;
const A: isize = 0; // BLUE (near)
const B: isize = 3; // GREEN (mid)
const C: isize = 6; // RED (far)


gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, (C * isz) as *const _); 
gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, (B * isz) as *const _); 
gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, (A * isz) as *const _); 

    // // Farthest triangles need to be drawn first: indices 0..3
    // gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, (0 * isz) as *const _);

    // // indices 3..6 added offset
    // gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, (3 * isz) as *const _);

    // //  indices 6..9
    // gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, (6 * isz) as *const _);

    gl::DepthMask(gl::TRUE);
            }

            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });

    // == //
    // == // From here on down there are only internals.
    // == //

    // Keep track of the health of the rendering thread
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

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                println!(
                    "New window size received: {}x{}",
                    physical_size.width, physical_size.height
                );
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        }
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Q => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => {}
        }
    });
}
