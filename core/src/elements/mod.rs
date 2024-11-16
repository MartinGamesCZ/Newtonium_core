use std::collections::HashMap;
use serde_json::Value;

pub mod button;
pub mod view;
pub mod text;
pub mod input;

// Function for getting the element creator
pub fn get_element_creator(
  tag: &str,
  args: HashMap<String, Value>
) -> Box<dyn Fn() -> gtk::Widget> {
  // Get creator function based on the tag
  let creator = match tag {
    "button" => button::element_create_button,
    "text" => text::create_element_text,
    "input" => input::create_element_input,
    "view" => view::create_element_view,

    // Panic if the element is unknown
    _ => panic!("Unknown element: {}", tag),
  };

  Box::new(move || creator(args.clone()))
}

// Function for setting the attribute of an element
pub fn set_element_attribute(tag: &str, element: &gtk::Widget, key: &str, value: &str) -> () {
  // Get the function for setting the attribute of the element
  let func: fn(&gtk::Widget, &str, &str) -> () = match tag {
    "text" => text::set_element_attribute_text,
    "button" => button::set_element_attribute_button,
    "input" => input::set_element_attribute_input,
    "view" => view::set_element_attribute_view,

    // Panic if the element is unknown
    _ => panic!("Unknown element: {}", tag),
  };

  // Set the attribute of the element
  func(element, key, value);

  ()
}

// Function for getting the attribute of an element
pub fn get_element_attribute(tag: &str, element: &gtk::Widget, key: &str) -> String {
  // Get the function for getting the attribute of the element
  let func: fn(&gtk::Widget, &str) -> String = match tag {
    "text" => text::get_element_attribute_text,
    "button" => button::get_element_attribute_button,
    "input" => input::get_element_attribute_input,
    "view" => view::get_element_attribute_view,

    // Panic if the element is unknown
    _ => panic!("Unknown element: {}", tag),
  };

  // Get the attribute of the element
  let value = func(element, key);

  value
}
