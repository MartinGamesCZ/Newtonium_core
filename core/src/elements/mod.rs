use std::collections::HashMap;

use gtk::Button;
use gtk::prelude::*;
use serde_json::Value;

pub mod button;
pub mod view;
pub mod text;

pub fn get_element_creator(
  tag: &str,
  args: HashMap<String, Value>
) -> Box<dyn Fn() -> gtk::Widget> {
  match tag {
    "button" => Box::new(|| button::element_create_button().upcast()),
    "view" => Box::new(|| view::create_element_view().upcast()),
    "text" => Box::new(move || text::create_element_text(args.clone()).upcast()),
    _ => panic!("Unknown element: {}", tag),
  }
}

pub fn set_element_attribute(tag: &str, element: &gtk::Widget, key: &str, value: &str) {
  match tag {
    "text" => text::set_element_attribute_text(element.downcast_ref().unwrap(), key, value),
    "button" => button::set_element_attribute_button(element.downcast_ref().unwrap(), key, value),
    _ => panic!("Unknown element: {}", tag),
  }
}
