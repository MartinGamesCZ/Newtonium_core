use std::ffi::CString;

use crate::GRAPHICS;

pub fn create_shader(src: &str, id: &str, vtype: gl::types::GLenum) -> () {
  GRAPHICS.with(|g| {
    let mut graphics = g.borrow_mut();

    let shader = unsafe {
      let shader = gl::CreateShader(vtype);
      let cstr = CString::new(src).unwrap();
      gl::ShaderSource(shader, 1, &cstr.as_ptr(), std::ptr::null());
      gl::CompileShader(shader);

      shader
    };

    graphics.insert(id.to_string(), shader);
  });
}

pub fn get_shader(id: &str) -> u32 {
  GRAPHICS.with(|g| {
    let graphics = g.borrow();
    graphics.get(id).unwrap().to_owned()
  })
}

pub fn delete_shader(id: &str) -> bool {
  GRAPHICS.with(|g| {
    let mut graphics = g.borrow_mut();
    let shader = graphics.remove(id);

    if shader.is_none() {
      return false;
    }

    unsafe {
      gl::DeleteShader(shader.unwrap());
    }

    true
  })
}
