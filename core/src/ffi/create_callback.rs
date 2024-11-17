use std::{ collections::HashMap, os::raw::c_char };
use std::sync::Mutex;

#[no_mangle]
// Function for creating a callback
extern "C" fn create_callback(
  callbacks_ptr: *mut Mutex<HashMap<String, extern "C" fn(*const c_char) -> ()>>,
  fun: extern "C" fn(*const c_char) -> (),
  id: *const c_char
) -> () {
  // Convert the pointer to a string
  let id = crate::utils::pointer::ptr_to_str(id);

  // Get the callbacks hashmap from the pointer
  let callbacks = unsafe { &mut *callbacks_ptr };

  // Insert the callback function into the hashmap
  callbacks.lock().unwrap().insert(id, fun);

  ()
}
