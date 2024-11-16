use crate::utils::element::get_element_by_id;
use gtk::prelude::*;

// [Command]
// Function for inserting an element before another element
pub fn insert_before(
  parent_id: &str,
  child_id: &str,
  sibling_id: &str,
  window: &gtk::Window
) -> () {
  // Get the parent, child and sibling elements
  let parent = get_element_by_id(parent_id);
  let child = get_element_by_id(child_id);
  let sibling = get_element_by_id(sibling_id);

  // Downcast parent to Container
  let parent = parent.downcast::<gtk::Container>().unwrap();

  // Get index of sibling in parent
  let index = parent
    .children()
    .iter()
    .position(|c| AsRef::<gtk::Widget>::as_ref(c) == AsRef::<gtk::Widget>::as_ref(&sibling))
    .unwrap();

  // Get everything after & including index
  let children_vec = parent.children();
  let children = children_vec.iter().skip(index);

  // Remove all children after index
  children.for_each(|c| {
    parent.remove(c);
  });

  // Add the child
  parent.add(&child);

  // Add all children after index
  children_vec.iter().for_each(|c| {
    parent.add(c);
  });

  // Update the window
  window.show_all();

  ()
}
