use std::collections::HashMap;

use gdk::{ glib::ObjectExt, RGBA };
use gtk::{ self, prelude::{ ButtonExt, CssProviderExt, EntryExt, StyleContextExt, WidgetExt } };
use serde_json::Value;

pub fn create_element_input(args: HashMap<String, Value>) -> gtk::Entry {
  let element = gtk::Entry::new();

  args.iter().for_each(|(key, value)| {
    set_element_attribute_input(&element, key, value.as_str().unwrap());
  });

  element
}

pub fn set_element_attribute_input(element: &gtk::Entry, key: &str, value: &str) {
  match key {
    "innerHTML" => element.set_text(value),
    _ => panic!("Unknown attribute: {}", key),
  }
}

pub fn get_element_attribute_input(element: &gtk::Entry, key: &str) -> String {
  match key {
    "innerHTML" => element.text().to_string(),
    _ => panic!("Unknown attribute: {}", key),
  }
}
