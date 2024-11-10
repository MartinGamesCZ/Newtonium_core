use std::collections::HashMap;

use serde_json::Value;

use crate::elements;

pub fn dom_create_element(tag: &str, args: HashMap<String, Value>) -> gtk::Widget {
  let creator = elements::get_element_creator(tag, args);

  creator()
}
