use std::{ ffi::CStr, os::raw::c_char };

use crate::{
  graphics::{ program::create_program, shader::create_shader },
  utils::channel::ptr_to_channel_tx,
};

#[no_mangle]
pub fn g_create_program(
  ptr: *mut async_channel::Sender<String>,
  id: *const c_char,
  vertex_shader: *const c_char,
  fragment_shader: *const c_char
) -> () {
  let id = unsafe { CStr::from_ptr(id).to_str().unwrap() };
  let vertex_shader = unsafe { CStr::from_ptr(vertex_shader).to_str().unwrap() };
  let fragment_shader = unsafe { CStr::from_ptr(fragment_shader).to_str().unwrap() };

  let tx = ptr_to_channel_tx(ptr);

  tx.unwrap()
    .try_send(format!("{};{};{};{}", "g_create_program", id, vertex_shader, fragment_shader))
    .unwrap();
}
