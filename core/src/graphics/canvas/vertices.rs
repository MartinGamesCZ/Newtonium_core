use gtk::prelude::*;
use crate::graphics::coordinates;

pub fn canvas_graphics_draw_vertices(
  program: u32,
  vertices: Vec<Vec<f32>>,
  color: [f32; 4], // RGBA
  canvas: &gtk::GLArea
) {
  // Remap vertices to GL coordinates
  let (canvas_width, canvas_height) = canvas.size_request();

  let vertices: Vec<[f32; 3]> = vertices
    .iter()
    .map(|vertex| {
      let x = coordinates::coordinate_to_gl(vertex[0] as i32, canvas_width);
      let y = coordinates::coordinate_to_gl(vertex[1] as i32, canvas_height);
      [x, y, vertex[2]]
    })
    .collect();

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
    gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as i32);

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
