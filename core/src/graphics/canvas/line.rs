use std::ffi::CString;

use gtk::prelude::*;
use crate::graphics::coordinates;

// Vertex shader
const VERTEX_SHADER: &str =
  r#"
    #version 330 core
    layout (location = 0) in vec2 position;
    void main() {
        gl_Position = vec4(position.x, position.y, 0.0, 1.0);
    }
"#;

// Fragment shader
const FRAGMENT_SHADER: &str =
  r#"
    #version 330 core
    uniform vec4 line_color;
    out vec4 FragColor;
    void main() {
        FragColor = line_color;
    }
"#;

pub fn canvas_graphics_draw_line(
  mprogram: u32,
  start_x: i32,
  start_y: i32,
  start_z: f32,
  end_x: i32,
  end_y: i32,
  end_z: f32,
  width: i32,
  color: [f32; 4], // RGBA
  canvas: &gtk::GLArea
) {
  let (canvas_width, canvas_height) = canvas.size_request();

  let start_x_gl = coordinates::coordinate_to_gl(start_x, canvas_width);
  let start_y_gl = coordinates::coordinate_to_gl(start_y, canvas_height);
  let end_x_gl = coordinates::coordinate_to_gl(end_x, canvas_width);
  let end_y_gl = coordinates::coordinate_to_gl(end_y, canvas_height);

  let vertices: [[f32; 3]; 2] = [
    [start_x_gl, start_y_gl, start_z],
    [end_x_gl, end_y_gl, end_z],
  ];

  unsafe {
    // Create vertex shader
    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    let vertex_source = CString::new(VERTEX_SHADER).unwrap();
    gl::ShaderSource(vertex_shader, 1, &vertex_source.as_ptr(), std::ptr::null());
    gl::CompileShader(vertex_shader);

    // Create fragment shader
    let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    let fragment_source = CString::new(FRAGMENT_SHADER).unwrap();
    gl::ShaderSource(fragment_shader, 1, &fragment_source.as_ptr(), std::ptr::null());
    gl::CompileShader(fragment_shader);

    // Create and link program
    let program = gl::CreateProgram();
    gl::AttachShader(program, vertex_shader);
    gl::AttachShader(program, fragment_shader);
    gl::LinkProgram(program);

    // Clean up shaders
    /*gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);*/

    // Set line width
    gl::LineWidth(width as f32);

    // Use shader program
    gl::UseProgram(program);

    // Set color uniform
    let color_loc = gl::GetUniformLocation(program, b"line_color\0".as_ptr() as *const i8);
    gl::Uniform4f(color_loc, color[0], color[1], color[2], color[3]);

    let vbo = new_array_buffer();
    let vao = new_vertex_array();
    gl::BindVertexArray(vao);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    buffer_dynamic_draw(&vertices);
    vertex_attrib_pointer();

    // Draw line
    gl::DrawArrays(gl::LINES, 0, 2);

    // Cleanup
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);
    gl::UseProgram(0);
  }
}

fn new_array_buffer() -> gl::types::GLuint {
  let mut vbo: gl::types::GLuint = 0;
  unsafe {
    gl::GenBuffers(1, &mut vbo);
  }
  vbo
}

fn new_vertex_array() -> gl::types::GLuint {
  let mut vao = 0;
  unsafe {
    gl::GenVertexArrays(1, &mut vao);
  }
  vao
}

fn buffer_dynamic_draw<T>(data: &[T]) {
  unsafe {
    gl::BufferData(
      gl::ARRAY_BUFFER,
      (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
      data.as_ptr() as *const gl::types::GLvoid,
      gl::DYNAMIC_DRAW
    );
  }
}

fn vertex_attrib_pointer() {
  unsafe {
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
      0,
      3,
      gl::FLOAT,
      gl::FALSE,
      (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
      std::ptr::null()
    );
  }
}
