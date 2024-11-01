use gtk::Button;

use crate::elements;

pub fn dom_create_element(tag: &str, args: Vec<String>) -> gtk::Widget {
  let creator = elements::get_element_creator(tag, args);

  creator()
}
