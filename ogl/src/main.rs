extern crate glfw;
use gl::types::{ GLfloat, GLsizei, GLsizeiptr, GLuint, GLvoid };
use glfw::{ GlfwReceiver, MouseButton, WindowEvent };

use self::glfw::{ Context, Key, Action };

extern crate gl;

use std::{ ffi::CString, sync::mpsc::Receiver };

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const vertex_shader_source: &str =
    r#"
#version 330 core
layout (location = 0) in vec3 aPos;
void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
    "#;

const fragment_shader_source: &str =
    r#"
#version 330 core
out vec4 FragColor;
void main() {
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
    "#;

pub fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init_no_callbacks().unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw
        .create_window(SCR_WIDTH, SCR_HEIGHT, "Hello World!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_mouse_button_callback(|_, _, action, _| {
        if action == Action::Press {
            println!("Mouse button pressed");
        }
    });

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader_program, VAO) = unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success = gl::FALSE as i32;
        let mut info_log = Vec::with_capacity(512);

        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character

        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        if success != (gl::TRUE as i32) {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut i8
            );
            println!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_shader);

        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != (gl::TRUE as i32) {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut i8
            );
            println!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != (gl::TRUE as i32) {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut i8
            );
            println!(
                "ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let vertices: [f32; 12] = [
            0.5,
            0.5,
            0.0, // Pravý horní roh
            0.5,
            -0.5,
            0.0, // Pravý spodní roh
            -0.5,
            -0.5,
            0.0, // Levý spodní roh
            -0.5,
            0.5,
            0.0, // Levý horní roh
        ];

        let indices: [u32; 6] = [
            0,
            1,
            3, // První trojúhelník
            1,
            2,
            3, // Druhý trojúhelník
        ];

        let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
        unsafe {
            // Vytvoření VAO
            gl::GenVertexArrays(1, &mut VAO);
            gl::BindVertexArray(VAO);

            // Vytvoření a naplnění VBO
            gl::GenBuffers(1, &mut VBO);
            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );

            // Vytvoření a naplnění EBO
            gl::GenBuffers(1, &mut EBO);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );

            // Specifikace vertex atributů
            gl::VertexAttribPointer(
                0, // Číslo atributu (odpovídá shaderu)
                3, // Počet komponent (x, y, z)
                gl::FLOAT, // Typ dat
                gl::FALSE, // Normalizace
                (3 * std::mem::size_of::<GLfloat>()) as GLsizei,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(0);

            // Unbindování VAO a VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            (shader_program, VAO)
        }
    };

    // render loop
    // -----------
    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events);
        //dbg!(&events);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(VAO);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &GlfwReceiver<(f64, WindowEvent)>) {
    for (_, event) in glfw::flush_messages(&events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe {
                    gl::Viewport(0, 0, width, height);
                }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) =>
                window.set_should_close(true),
            glfw::WindowEvent::Key(key, _, _, _) => {
                window.set_title(key.get_name().unwrap().as_str());
            }
            glfw::WindowEvent::MouseButton(_, _, _) => {
                println!("Mouse button event");
            }
            _ => {}
        }
    }
}
