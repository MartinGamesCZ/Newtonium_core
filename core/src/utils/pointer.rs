use std::{ ffi::CStr, os::raw::c_char };

// Function for converting a pointer to a string
pub fn ptr_to_str(ptr: *const c_char) -> String {
  // Convert the pointer to a C string
  let c_str = unsafe { CStr::from_ptr(ptr) };

  // Convert the C string to String
  let str = c_str.to_str().unwrap().to_string();

  str
}
