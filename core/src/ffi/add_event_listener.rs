use std::{ collections::HashMap, os::raw::c_char };
use crate::utils::{ channel::frame_to_ptr, pointer::ptr_to_str };
use gdk::glib::Value;
use gtk::prelude::*;
use std::sync::Mutex;

#[no_mangle]
// Function for adding an event listener
pub extern "C" fn add_event_listener(
  elements_ptr: *mut Mutex<HashMap<String, gtk::Widget>>,
  callbacks_ptr: *mut Mutex<HashMap<String, extern "C" fn(*const c_char) -> ()>>,
  id: *const c_char,
  key: *const c_char,
  callback_id: *const c_char,
  symbol_id: *const c_char
) -> () {
  // Convert the pointers to strings
  let id = ptr_to_str(id);
  let key = ptr_to_str(key);

  // Get elements hashmap pointer
  let elements = unsafe { &mut *elements_ptr };

  // Get the element from the hashmap
  let guard = elements.lock().unwrap();
  let element = guard.get(&id);

  // If the element is not found, return
  if element.is_none() {
    println!("Element with ID {} not found", id);
    return ();
  }

  // Clone the element
  let element = element.unwrap().clone();

  // Get the callback ID from pointer
  let callback_id = ptr_to_str(callback_id);

  // Get the callback function from the hashmap
  let callbacks = unsafe { &mut *callbacks_ptr };

  // Get the callback function from the hashmap
  let guard = callbacks.lock().unwrap();
  let cb = guard.get(&callback_id);

  // If the callback function is not found, return
  if cb.is_none() {
    println!("Callback with ID {} not found", callback_id);
    return ();
  }

  // Clone the callback function
  let cb = cb.unwrap().clone();

  // Convert the symbol ID to a string
  let symbol_id = ptr_to_str(symbol_id);

  // Connect the event listener
  element.connect(&key.clone(), false, move |_| {
    // Call the callback function
    cb(frame_to_ptr(&symbol_id));

    match key.as_str() {
      "render" => Some(Value::from(true)),
      _ => None,
    }
  });

  ()
}
