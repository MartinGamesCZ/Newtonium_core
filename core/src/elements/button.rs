use gdk::glib::ObjectExt;
use gtk::{ self, prelude::ButtonExt };

pub fn element_create_button() -> gtk::Button {
  let button = gtk::Button::with_label("");

  button
}

pub fn set_element_attribute_button(element: &gtk::Button, key: &str, value: &str) {
  match key {
    "innerHTML" => element.set_label(value),
    _ => panic!("Unknown attribute: {}", key),
  }
}
