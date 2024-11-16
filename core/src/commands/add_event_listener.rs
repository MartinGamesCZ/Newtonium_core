use std::os::raw::c_char;

use crate::utils::element::get_element_by_id;
use gtk::prelude::*;

// [Command]
// Function for adding an event listener to an element
pub fn add_event_listener(
  id: &str,
  event: &str,
  symbol_id: &str,
  event_cb: extern "C" fn(*const c_char, *const c_char) -> ()
) -> () {
  // Get the element by its ID
  let element = get_element_by_id(id);

  // Change data types to prevent lifetime issues
  let id = id.to_string();
  let symbol_id = symbol_id.to_string();

  // Connect the event to the element
  element.connect(event, false, move |_| {
    // Call the event callback with corresponding symbol ID and element ID
    event_cb(symbol_id.as_ptr() as *const i8, id.as_ptr() as *const i8);

    None
  });

  ()
}
