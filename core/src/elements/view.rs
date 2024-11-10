use std::collections::HashMap;

use gtk::{ self, prelude::{ OrientableExt } };
use serde_json::Value;

pub fn create_element_view(args: HashMap<String, Value>) -> gtk::Box {
  let element = gtk::Box::new(gtk::Orientation::Vertical, 0);

  args.iter().for_each(|(key, value)| {
    set_element_attribute_view(&element, key, value.as_str().unwrap());
  });

  element
}

pub fn set_element_attribute_view(element: &gtk::Box, key: &str, value: &str) {
  match key {
    "direction" =>
      element.set_orientation(match value {
        "vertical" => gtk::Orientation::Vertical,
        "horizontal" => gtk::Orientation::Horizontal,
        _ => panic!("Unknown orientation: {}", value),
      }),
    _ => panic!("Unknown attribute: {}", key),
  }
}

pub fn get_element_attribute_view(element: &gtk::Box, key: &str) -> String {
  match key {
    "direction" =>
      match element.orientation() {
        gtk::Orientation::Vertical => "vertical".to_string(),
        gtk::Orientation::Horizontal => "horizontal".to_string(),
        _ => panic!("Unknown orientation: {:?}", element.orientation()),
      }
    _ => panic!("Unknown attribute: {}", key),
  }
}
