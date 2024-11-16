use crate::{ elements::set_element_attribute, utils::element::get_element_by_id };

// [Command]
// Function for setting an attribute of an element
pub fn set_attribute(id: &str, tag: &str, key: &str, value: &str, window: &gtk::Window) -> () {
  // Get the element by its ID
  let element = get_element_by_id(id);

  // Set the attribute of the element
  set_element_attribute(tag, &element, key, value, window);

  ()
}
