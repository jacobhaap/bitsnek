mod models;

use std::{ffi::c_void, mem, ptr, sync::mpsc::Receiver, time::Instant};

use cgmath::{Matrix4, vec3};
use glfw::{Action, Context, Key, WindowEvent};
use models::{core::game::Game};
use crate::models::{opengl::{camera::Camera, shader::Shader, text_renderer::TextRenderer, texture::Texture}};
use gl::types::*;

const WIDTH: u32 = 1500;
const HEIGHT: u32 = 1000;

fn main() {
    // wrap program in helper
    // for unsafe block w/o indentation
    unsafe { start(); }
}

unsafe fn start() {
    // glfw: initialize
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true)); 

    // glfw window creation
    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, "BitSnek", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_scroll_polling(true);
    window.set_mouse_button_polling(true);
    window.set_title("BitSnek");

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // depth buffer
    gl::Enable(gl::DEPTH_TEST);

    let shader = Shader::new("assets/shaders/vertex.vert", "assets/shaders/fragment.frag");
    let apple_shader = Shader::new("assets/shaders/apple_vertex.vert", "assets/shaders/apple_fragment.frag");
    
    // create vertex array
    let mut vao = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    let mut vbo = 0;
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

    // set vertex attribute pointers
    // position
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<GLfloat>() as GLsizei, (3 * std::mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);

    // create second vertex array for apple
    let mut apple_vao = 0;
    gl::GenVertexArrays(1, &mut apple_vao);
    gl::BindVertexArray(apple_vao);

    let mut apple_vbo = 0;
    gl::GenBuffers(1, &mut apple_vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, apple_vbo);

    // set vertex attribute pointers
    // position
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<GLfloat>() as GLsizei, (3 * std::mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);

    let mut camera = Camera::new(WIDTH, HEIGHT, 0.008);
    camera.position.y = 20.0;

    let mut instant = Instant::now();
    let mut tick_timer = Instant::now();

    let mut last_x = 0.0;
    let mut last_y = 0.0;
    let mut first_mouse = true;

    let text_renderer = TextRenderer::new(
        WIDTH, 
        HEIGHT, 
        "assets/font/OldSchoolAdventures.ttf", 
        "assets/shaders/text_vertex.vert", 
        "assets/shaders/text_fragment.frag",
    );
    let texture_map = Texture::new(
        "assets/textures/textures.png", 
        gl::TEXTURE0, 
        false
    );

    // target fps
    let target_fps = 60.0;

    // create game object
    let mut game = Game::new(16, 16);
    game.update();

    gl::BindVertexArray(vao);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER, 
        (mem::size_of::<f32>() * game.vertices.len()) as isize,
        game.vertices.as_ptr() as *const c_void, 
        gl::DYNAMIC_DRAW
    );

    gl::BindVertexArray(apple_vao);
    gl::BindBuffer(gl::ARRAY_BUFFER, apple_vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER, 
        (mem::size_of::<f32>() * game.apple_vertices.len()) as isize,
        game.apple_vertices.as_ptr() as *const c_void, 
        gl::DYNAMIC_DRAW
    );

    let mut t = 0.02;
    // render loop
    while !window.should_close() {
        let deltatime = instant.elapsed().as_millis() as f32;
        instant = Instant::now();

        t += 0.01;

        // events
        process_events(
            &mut window, 
            &events, 
            &mut camera, 
            &mut game,
            &mut last_x, 
            &mut last_y, 
            &mut first_mouse
        );
        camera.update_position(deltatime);

        // clear buffers
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 

        // draw text
        text_renderer.render_text(format!("FPS: {}", (1000.0 / deltatime).round()).as_str(), 10.0, (HEIGHT as f32) - 30.0, 1.0, vec3(1.0, 0.0, 0.0));
        text_renderer.render_text(format!("x: {:.0}", camera.position.x).as_str(), 10.0, (HEIGHT as f32) - 50.0, 0.6, vec3(1.0, 0.0, 0.0));
        text_renderer.render_text(format!("y: {:.0}", camera.position.y).as_str(), 10.0, (HEIGHT as f32) - 70.0, 0.6, vec3(1.0, 0.0, 0.0));
        text_renderer.render_text(format!("z: {:.0}", camera.position.z).as_str(), 10.0, (HEIGHT as f32) - 90.0, 0.6, vec3(1.0, 0.0, 0.0));

        // bind texture
        texture_map.bind();
        shader.set_texture("texture_map", &texture_map);

        // draw
        if (tick_timer.elapsed().as_millis() as f32) > 200.0 {
            tick_timer = Instant::now();
            game.update();
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (mem::size_of::<f32>() * game.vertices.len()) as isize,
                game.vertices.as_ptr() as *const c_void, 
                gl::DYNAMIC_DRAW
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, apple_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (mem::size_of::<f32>() * game.apple_vertices.len()) as isize,
                game.apple_vertices.as_ptr() as *const c_void, 
                gl::STATIC_DRAW
            );
        }

        println!("binding vao and rendering");
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        shader.use_program();
        shader.set_mat4("view", camera.get_view());
        shader.set_mat4("projection", camera.get_projection());
        shader.set_mat4("model", Matrix4::<f32>::from_nonuniform_scale(3.0, 1.0, 1.0));
        shader.set_float("time", t);

        gl::DrawArrays(gl::TRIANGLES, 0, game.vertices.len() as GLint); 
        println!("Rendered");

        gl::BindVertexArray(apple_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, apple_vbo);
        apple_shader.use_program();
        apple_shader.set_mat4("view", camera.get_view());
        apple_shader.set_mat4("projection", camera.get_projection());
        shader.set_mat4("model", Matrix4::<f32>::from_nonuniform_scale(3.0, 1.0, 1.0));
        apple_shader.set_float("time", t);

        println!("Drawing apple VAO");
        gl::DrawArrays(gl::TRIANGLES, 0, game.apple_vertices.len() as GLint);
        println!("Finished drawing");

        window.swap_buffers();
        glfw.poll_events();

        // hang thread for target FPS
        while (instant.elapsed().as_millis() as f32) < (1000.0 / target_fps) {}
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, camera: &mut Camera, game: &mut Game, last_x: &mut f32, last_y: &mut f32, first_mouse: &mut bool) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            },
            WindowEvent::Scroll(_, y_offset) => {
                camera.scroll_callback(y_offset as f32);
            },
            WindowEvent::CursorPos(xpos, ypos) => {
                let (x_pos, y_pos) = (xpos as f32, ypos as f32);
                let x_offset = x_pos - *last_x;
                let y_offset = *last_y - y_pos;
                *last_x = x_pos;
                *last_y = y_pos;
                if *first_mouse {
                    *first_mouse = false;
                    return;
                }

                camera.mouse_callback(x_offset, y_offset);
            },
            WindowEvent::Key(Key::LeftShift, _, Action::Press, _) => camera.speed = 0.05,
            WindowEvent::Key(Key::LeftShift, _, Action::Release, _) => camera.speed = 0.008,
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            WindowEvent::Key(key, _, action, _) => {
                camera.process_keyboard(key, action);
                game.process_keyboard(key, action);
            },
            _ => ()
        }
    }
}
