use std::collections::HashMap;

use gdk::{ glib::ObjectExt, RGBA };
use gtk::{
  self,
  prelude::{ ButtonExt, CssProviderExt, StyleContextExt, WidgetExt },
  STYLE_PROPERTY_BACKGROUND_COLOR,
  STYLE_PROPERTY_PADDING,
};
use serde_json::Value;

pub fn element_create_button(args: HashMap<String, Value>) -> gtk::Button {
  let element = gtk::Button::with_label("");

  args.iter().for_each(|(key, value)| {
    set_element_attribute_button(&element, key, value.as_str().unwrap());
  });

  element
}

pub fn set_element_attribute_button(element: &gtk::Button, key: &str, value: &str) {
  match key {
    "innerHTML" => element.set_label(value),
    _ => panic!("Unknown attribute: {}", key),
  }
}

pub fn get_element_attribute_button(element: &gtk::Button, key: &str) -> String {
  match key {
    "innerHTML" => element.label().unwrap().to_string(),
    _ => panic!("Unknown attribute: {}", key),
  }
}
