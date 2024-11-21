use std::ffi::CString;

use gtk::prelude::*;
use crate::graphics::coordinates;

pub fn canvas_graphics_draw_rectangle(
  program: u32,
  start_x: i32,
  start_y: i32,
  start_z: f32,
  end_x: i32,
  end_y: i32,
  end_z: f32,
  color: [f32; 4], // RGBA
  canvas: &gtk::GLArea
) {
  let (canvas_width, canvas_height) = canvas.size_request();

  let start_x_gl = coordinates::coordinate_to_gl(start_x, canvas_width);
  let start_y_gl = coordinates::coordinate_to_gl(start_y, canvas_height);
  let end_x_gl = coordinates::coordinate_to_gl(end_x, canvas_width);
  let end_y_gl = coordinates::coordinate_to_gl(end_y, canvas_height);

  // Define vertices for a rectangle (6 vertices for 2 triangles)
  let vertices: [[f32; 3]; 6] = [
    // First triangle
    [start_x_gl, start_y_gl, start_z], // Bottom-left
    [end_x_gl, start_y_gl, start_z], // Bottom-right
    [start_x_gl, end_y_gl, start_z], // Top-left
    // Second triangle
    [end_x_gl, start_y_gl, end_z], // Bottom-right
    [end_x_gl, end_y_gl, end_z], // Top-right
    [start_x_gl, end_y_gl, end_z], // Top-left
  ];

  unsafe {
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

    // Draw rectangle using triangles
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

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
