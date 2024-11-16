use crate::utils::element::get_element_by_id;
use gtk::prelude::*;

// [Command]
// Function for appending a child to a parent
pub fn append_child(parent_id: &str, child_id: &str, window: &gtk::Window) -> () {
  // Get the parent and child elements
  let parent = get_element_by_id(parent_id);
  let child = get_element_by_id(child_id);

  // Downcast the parent to a container
  let parent = parent.downcast::<gtk::Container>().unwrap();

  // Add the child to the parent
  parent.add(&child);

  // Update the window
  window.show_all();

  ()
}
