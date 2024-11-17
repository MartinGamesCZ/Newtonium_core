use std::{ collections::HashMap, os::raw::c_char };
use crate::{
  elements::get_element_attribute,
  utils::{ channel::frame_to_ptr, pointer::ptr_to_str },
};

#[no_mangle]
// Function for getting an attribute of an element
pub extern "C" fn get_attribute(
  elements_ptr: *mut std::sync::Mutex<HashMap<String, gtk::Widget>>,
  id: *const c_char,
  tag: *const c_char,
  key: *const c_char
) -> *const c_char {
  // Convert the pointers to strings
  let id = ptr_to_str(id);
  let tag = ptr_to_str(tag);
  let key = ptr_to_str(key);

  // Get elements hashmap from the pointer
  let elements = unsafe { &mut *elements_ptr };

  // Get the element from the hashmap
  let guard = elements.lock().unwrap();
  let element = guard.get(&id);

  // If the element is not found, return
  if element.is_none() {
    println!("Element with ID {} not found", id);
    return "".as_ptr() as *const c_char;
  }

  // Clone the element
  let element = element.unwrap().clone();

  // Get the value of the attribute
  let value = get_element_attribute(&tag, &element, &key);

  // Frame the data and return the pointer
  frame_to_ptr(&value)
}
