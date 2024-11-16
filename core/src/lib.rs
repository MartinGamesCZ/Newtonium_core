use std::{ cell::RefCell, collections::HashMap };

pub mod elements;
pub mod utils;
pub mod ffi;
pub mod commands;
pub mod units;

// Thread local storage for windows, elements, and element styles
thread_local! {
  pub static WINDOWS: RefCell<HashMap<String, gtk::Window>> = RefCell::new(HashMap::new());
  pub static ELEMENTS: RefCell<HashMap<String, gtk::Widget>> = RefCell::new(HashMap::new());
  pub static ELEMENT_STYLES: RefCell<HashMap<String, gtk::CssProvider>> = RefCell::new(
    HashMap::new()
  );
}
