use std::collections::HashMap;

use gtk::prelude::*;
use serde_json::Value;

pub fn create_element_text(args: HashMap<String, Value>) -> gtk::Label {
  let element = gtk::Label::new(None);

  args.iter().for_each(|(key, value)| {
    set_element_attribute_text(&element, key, value.as_str().unwrap());
  });

  element
}

pub fn set_element_attribute_text(element: &gtk::Label, key: &str, value: &str) {
  match key {
    "innerHTML" => element.set_label(value),
    _ => panic!("Unknown attribute: {}", key),
  }
}

pub fn get_element_attribute_text(element: &gtk::Label, key: &str) -> String {
  match key {
    "innerHTML" => element.label().to_string(),
    _ => panic!("Unknown attribute: {}", key),
  }
}
