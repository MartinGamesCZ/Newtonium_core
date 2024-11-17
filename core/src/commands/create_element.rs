use std::collections::HashMap;
use serde_json::Value;
use gtk::prelude::*;
use crate::{ elements::get_element_creator, ELEMENTS };

// [Command]
// Function for creating a new element
pub fn create_element(id: &str, tag: &str, args: &str, window: &gtk::Window) -> () {
  // Parse the arguments
  // JSON -> HashMap
  let output: HashMap<String, Value> = serde_json::from_str(args).unwrap();

  // Create the element
  let creator = get_element_creator(tag, output, window);
  let element = creator();

  // Add the class to the element
  element.style_context().add_class(("iid_".to_string() + id).as_str());

  // Add the element to the elements list
  ELEMENTS.with(|elements| {
    // Get elements list lock and insert the element
    elements.lock().unwrap().insert(id.to_string(), element);
  });

  ()
}
