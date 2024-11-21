use std::collections::HashMap;
use gtk::prelude::*;
use serde_json::Value;
use webkit2gtk::{ WebView, WebViewExt };

use crate::units::length::length_to_px;

// Function for creating a new webview element
// Webview -> WebView (webkit2gtk)
pub fn create_element_webview(args: HashMap<String, Value>, window: gtk::Window) -> gtk::Widget {
  // Create a new WebView element
  let element = WebView::new();

  // Set the initial attributes of the element
  args.iter().for_each(|(key, value)| {
    // Upcast the element to a GtkWidget
    let upcasted_element = element.upcast_ref::<gtk::Widget>();
    let str_value = value.as_str().unwrap();

    // Set the attribute of the element
    set_element_attribute_webview(upcasted_element, key, str_value, &window);
  });

  element.upcast()
}

// Function for setting the attribute of a webview element
pub fn set_element_attribute_webview(
  element: &gtk::Widget,
  key: &str,
  value: &str,
  window: &gtk::Window
) -> () {
  // Downcast the element to a WebView
  let downcasted_element = element.downcast_ref::<WebView>().unwrap();

  // Get parent element
  let parent = downcasted_element.parent();

  // Set the attribute of the element
  match key {
    "uri" => downcasted_element.load_uri(value),
    "width" =>
      downcasted_element.set_width_request(
        length_to_px(
          value,
          match parent {
            Some(parent) => parent.width_request(),
            None => window.width_request(),
          },
          window
        )
      ),
    "height" =>
      downcasted_element.set_height_request(
        length_to_px(
          value,
          match parent {
            Some(parent) => parent.height_request(),
            None => window.height_request(),
          },
          window
        )
      ),

    // Panic if the attribute is unknown
    _ => panic!("Unknown attribute: {}", key),
  }

  ()
}

// Function for getting the attribute of a webview element
pub fn get_element_attribute_webview(element: &gtk::Widget, key: &str) -> String {
  // Downcast the element to a WebView
  let element = element.downcast_ref::<WebView>().unwrap();

  // Get the attribute of the element
  let value = match key {
    "uri" => element.uri().unwrap().to_string(),
    "width" => element.width_request().to_string(),
    "height" => element.height_request().to_string(),

    // Panic if the attribute is unknown
    _ => panic!("Unknown attribute: {}", key),
  };

  value
}