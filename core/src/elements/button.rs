use gdk::{ glib::ObjectExt, RGBA };
use gtk::{
  self,
  prelude::{ ButtonExt, CssProviderExt, StyleContextExt, WidgetExt },
  STYLE_PROPERTY_BACKGROUND_COLOR,
  STYLE_PROPERTY_PADDING,
};

pub fn element_create_button() -> gtk::Button {
  let button = gtk::Button::with_label("");

  let css_provider = gtk::CssProvider::new();
  css_provider.load_from_data(b"button { background-color: red; }").expect("Failed to load CSS");
  button.style_context().add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

  let cln = button.clone();
  button.connect_clicked(move |_| {
    cln.style_context().remove_provider(&css_provider);

    let css_provider = gtk::CssProvider::new();
    css_provider
      .load_from_data(b"button { background-color: green; }")
      .expect("Failed to load CSS");

    cln.style_context().add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
  });

  button
}

pub fn set_element_attribute_button(element: &gtk::Button, key: &str, value: &str) {
  match key {
    "innerHTML" => element.set_label(value),
    _ => panic!("Unknown attribute: {}", key),
  }
}
