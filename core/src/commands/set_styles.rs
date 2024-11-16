use crate::utils::element::{
  get_element_by_id,
  get_element_styles_provider,
  set_element_styles_provider,
};
use gtk::prelude::*;

// [Command]
// Function for setting the styles of an element
pub fn set_styles(id: &str, styles: &str) -> () {
  // Get the element by its ID
  let element = get_element_by_id(id);

  // Get the old provider
  let old_provider = get_element_styles_provider(id);

  // Remove the old provider, if it exists
  if old_provider.is_some() {
    element.style_context().remove_provider(&old_provider.unwrap());
  }

  // Create a new provider
  let provider = gtk::CssProvider::new();

  // Decode the styles (replaced ; for ~ because of command splitting)
  let styles = styles.replace("~", ";");

  // Load the styles
  provider.load_from_data(styles.as_bytes()).unwrap();

  // Add the provider to the element
  element.style_context().add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

  // Set the provider
  set_element_styles_provider(id, provider);

  ()
}
