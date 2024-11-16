use std::collections::HashMap;
use gtk::prelude::*;
use serde_json::Value;

// Function for creating a new view element
// View -> GtkBox
pub fn create_element_view(args: HashMap<String, Value>) -> gtk::Widget {
  // Create a new GtkBox element
  let element = gtk::Box::new(gtk::Orientation::Vertical, 0);

  // Set the initial attributes of the element
  args.iter().for_each(|(key, value)| {
    // Upcast the element to a GtkWidget
    let upcasted_element = element.upcast_ref::<gtk::Widget>();
    let str_value = value.as_str().unwrap();

    // Set the attribute of the element
    set_element_attribute_view(upcasted_element, key, str_value);
  });

  element.upcast()
}

// Function for setting the attribute of a view element
pub fn set_element_attribute_view(element: &gtk::Widget, key: &str, value: &str) -> () {
  // Downcast the element to a GtkBox
  let element = element.downcast_ref::<gtk::Box>().unwrap();

  // Set the attribute of the element
  match key {
    "direction" =>
      element.set_orientation(match value {
        "vertical" => gtk::Orientation::Vertical,
        "horizontal" => gtk::Orientation::Horizontal,

        // Panic if the orientation is unknown
        _ => panic!("Unknown orientation: {}", value),
      }),
    "spacing" => element.set_spacing(value.parse().unwrap()),
    "width" => element.set_width_request(value.parse().unwrap()),
    "height" => element.set_height_request(value.parse().unwrap()),

    // Panic if the attribute is unknown
    _ => panic!("Unknown attribute: {}", key),
  }

  ()
}

// Function for getting the attribute of a view element
pub fn get_element_attribute_view(element: &gtk::Widget, key: &str) -> String {
  // Downcast the element to a GtkBox
  let element = element.downcast_ref::<gtk::Box>().unwrap();

  // Get the attribute of the element
  let value = match key {
    "direction" =>
      match element.orientation() {
        gtk::Orientation::Vertical => "vertical".to_string(),
        gtk::Orientation::Horizontal => "horizontal".to_string(),

        // Panic if the orientation is unknown
        _ => panic!("Unknown orientation: {:?}", element.orientation()),
      }
    "spacing" => element.spacing().to_string(),
    "width" => element.width_request().to_string(),
    "height" => element.height_request().to_string(),

    // Panic if the attribute is unknown
    _ => panic!("Unknown attribute: {}", key),
  };

  value
}
