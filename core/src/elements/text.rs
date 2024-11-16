use std::collections::HashMap;
use gtk::prelude::*;
use serde_json::Value;

// Function for creating a new text element
// Text -> GtkLabel
pub fn create_element_text(args: HashMap<String, Value>) -> gtk::Widget {
  // Create a new GtkLabel element
  let element = gtk::Label::new(None);

  // Set the initial attributes of the element
  args.iter().for_each(|(key, value)| {
    // Upcast the element to a GtkWidget
    let upcasted_element = element.upcast_ref::<gtk::Widget>();
    let str_value = value.as_str().unwrap();

    // Set the attribute of the element
    set_element_attribute_text(upcasted_element, key, str_value);
  });

  element.upcast()
}

// Function for setting the attribute of a text element
pub fn set_element_attribute_text(element: &gtk::Widget, key: &str, value: &str) -> () {
  // Downcast the element to a GtkLabel
  let downcasted_element = element.downcast_ref::<gtk::Label>().unwrap();

  // Set the attribute of the element
  match key {
    "innerHTML" => downcasted_element.set_label(value),
    "width" => downcasted_element.set_width_request(value.parse().unwrap()),
    "height" => downcasted_element.set_height_request(value.parse().unwrap()),

    // Panic if the attribute is unknown
    _ => panic!("Unknown attribute: {}", key),
  }

  ()
}

// Function for getting the attribute of a text element
pub fn get_element_attribute_text(element: &gtk::Widget, key: &str) -> String {
  // Downcast the element to a GtkLabel
  let downcasted_element = element.downcast_ref::<gtk::Label>().unwrap();

  // Get the attribute of the element
  let value = match key {
    "innerHTML" => downcasted_element.label().to_string(),
    "width" => downcasted_element.width_request().to_string(),
    "height" => downcasted_element.height_request().to_string(),

    // Panic if the attribute is unknown
    _ => panic!("Unknown attribute: {}", key),
  };

  value
}
