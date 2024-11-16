use crate::{ ELEMENTS, ELEMENT_STYLES };
use gtk::prelude::*;

// Function for getting an element by its ID
pub fn get_element_by_id(id: &str) -> gtk::Widget {
  // Get the element by its ID
  let element = ELEMENTS.with(|elements| { elements.borrow().get(id).unwrap().clone() });

  element
}

// Function for removing an element from list
pub fn remove_element_from_list(id: &str) -> () {
  ELEMENTS.with(|elements| {
    elements.borrow_mut().remove(id);
  });

  ()
}

// Function for getting element's parent
pub fn get_element_parent<T: gtk::prelude::IsA<gtk::Widget>>(element: &gtk::Widget) -> T {
  // Get the parent of the element
  let parent = element.parent().unwrap().clone().downcast::<T>().unwrap();

  parent
}

// Function for getting element's styles provider
pub fn get_element_styles_provider(id: &str) -> Option<gtk::CssProvider> {
  let provider = ELEMENT_STYLES.with(|element_styles| {
    let styles = element_styles.borrow();
    let provider: Option<gtk::CssProvider> = styles.get(id).cloned();

    provider
  });

  provider
}

// Function for setting element's styles provider
pub fn set_element_styles_provider(id: &str, provider: gtk::CssProvider) -> () {
  ELEMENT_STYLES.with(|styles| {
    styles.borrow_mut().insert(id.to_string(), provider.clone());
  });

  ()
}
