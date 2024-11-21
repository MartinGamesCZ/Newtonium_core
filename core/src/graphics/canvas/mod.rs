use gtk::prelude::*;

use crate::graphics::{ program::create_program, shader::create_shader };

use super::shader::delete_shader;

pub mod clear;
pub mod line;
pub mod rectangle;

const VERTEX_SHADER: &str =
  r#"
    #version 330 core
    layout (location = 0) in vec2 position;
    void main() {
        gl_Position = vec4(position.x, position.y, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str =
  r#"
    #version 330 core
    uniform vec4 line_color;
    out vec4 FragColor;
    void main() {
        FragColor = line_color;
    }
"#;

pub fn canvas_load() -> () {
  #[cfg(target_os = "macos")]
  let library = (unsafe { libloading::os::unix::Library::new("libepoxy.0.dylib") }).unwrap();
  #[cfg(all(unix, not(target_os = "macos")))]
  let library = (unsafe { libloading::os::unix::Library::new("libepoxy.so.0") }).unwrap();
  #[cfg(windows)]
  let library = libloading::os::windows::Library
    ::open_already_loaded("libepoxy-0.dll")
    .or_else(|_| libloading::os::windows::Library::open_already_loaded("epoxy-0.dll"))
    .unwrap();

  epoxy::load_with(|name| {
    (unsafe { library.get::<_>(name.as_bytes()) }).map(|symbol| *symbol).unwrap_or(std::ptr::null())
  });
  gl::load_with(|name| { epoxy::get_proc_addr(name) });
}

pub fn canvas_make_current(canvas: &gtk::GLArea) -> () {
  canvas.make_current();
}

pub fn canvas_create_program() {
  create_shader(VERTEX_SHADER, "@g_line_vertex", gl::VERTEX_SHADER);
  create_shader(FRAGMENT_SHADER, "@g_line_fragment", gl::FRAGMENT_SHADER);

  create_program("@g_line_vertex", "@g_line_fragment", "@g_line");

  delete_shader("@g_line_vertex");
  delete_shader("@g_line_fragment");
}
