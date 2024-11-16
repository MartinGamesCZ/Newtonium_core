use std::collections::HashMap;
use gtk::prelude::*;
use serde_json::Value;

use crate::units::length::length_to_px;

// Function for creating a new view element
// View -> GtkBox
pub fn create_element_view(args: HashMap<String, Value>, window: gtk::Window) -> gtk::Widget {
  // Create a new GtkBox element
  let element = gtk::Box::new(gtk::Orientation::Vertical, 0);

  // Set the initial attributes of the element
  args.iter().for_each(|(key, value)| {
    // Upcast the element to a GtkWidget
    let upcasted_element = element.upcast_ref::<gtk::Widget>();
    let str_value = value.as_str().unwrap();

    // Set the attribute of the element
    set_element_attribute_view(upcasted_element, key, str_value, &window);
  });

  element.upcast()
}

// Function for setting the attribute of a view element
pub fn set_element_attribute_view(
  element: &gtk::Widget,
  key: &str,
  value: &str,
  window: &gtk::Window
) -> () {
  // Downcast the element to a GtkBox
  let downcasted_element = element.downcast_ref::<gtk::Box>().unwrap();

  // Get parent element
  let parent = downcasted_element.parent();

  // Set the attribute of the element
  match key {
    "direction" =>
      downcasted_element.set_orientation(match value {
        "vertical" => gtk::Orientation::Vertical,
        "horizontal" => gtk::Orientation::Horizontal,

        // Panic if the orientation is unknown
        _ => panic!("Unknown orientation: {}", value),
      }),
    "spacing" => downcasted_element.set_spacing(value.parse().unwrap()),
    "width" =>
      downcasted_element.set_width_request(
        length_to_px(
          value,
          match parent {
            Some(parent) => parent.width_request(),
            None => window.width_request(),
          },
          window
        )
      ),
    "height" =>
      downcasted_element.set_width_request(
        length_to_px(
          value,
          match parent {
            Some(parent) => parent.width_request(),
            None => window.width_request(),
          },
          window
        )
      ),

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
