use std::os::raw::c_char;

use crate::{ elements::get_element_attribute, utils::element::get_element_by_id };

// [Command]
// Function for setting the styles of an element
pub fn get_attribute(
  id: &str,
  tag: &str,
  key: &str,
  symbol_id: &str,
  event_cb: extern "C" fn(*const c_char, *const c_char) -> ()
) -> () {
  // Get the element by its ID
  let element = get_element_by_id(id);

  // Get the value of the attribute
  let value = get_element_attribute(tag, &element, key);

  // Send the value back using special event
  let response = format!("{}!!{};~;;", symbol_id, value);
  event_cb("!!get_property".as_ptr() as *const i8, response.as_ptr() as *const i8);

  ()
}
