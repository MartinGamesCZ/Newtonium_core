use gtk::prelude::*;

pub fn create_element_text(args: Vec<String>) -> gtk::Label {
  let element = gtk::Label::new(None);

  element
}

pub fn set_element_attribute_text(element: &gtk::Label, key: &str, value: &str) {
  match key {
    "innerHTML" => element.set_label(value),
    _ => panic!("Unknown attribute: {}", key),
  }
}
