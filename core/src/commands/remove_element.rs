use crate::utils::element::{ get_element_by_id, get_element_parent, remove_element_from_list };
use gtk::prelude::*;

// [Command]
// Function for removing an element
pub fn remove_element(id: &str) -> () {
  // Get the element by its ID
  let element = get_element_by_id(id);

  // Get the parent of the element
  let parent = get_element_parent::<gtk::Container>(&element);

  // Remove the element from the parent
  parent.remove(&element);

  // Remove the element from the list
  remove_element_from_list(id);

  ()
}
