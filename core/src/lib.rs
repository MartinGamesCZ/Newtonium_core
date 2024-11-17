use std::{ cell::RefCell, collections::HashMap, os::raw::c_char };

pub mod elements;
pub mod utils;
pub mod ffi;
pub mod commands;
pub mod units;

// Thread local storage for windows, elements, and element styles
thread_local! {
  pub static WINDOWS: RefCell<HashMap<String, gtk::Window>> = RefCell::new(HashMap::new());
  pub static ELEMENTS: std::sync::Mutex<HashMap<String, gtk::Widget>> = std::sync::Mutex::new(
    HashMap::new()
  );
  pub static CALLBACKS: std::sync::Mutex<
    HashMap<String, extern "C" fn(*const c_char) -> ()>
  > = std::sync::Mutex::new(HashMap::new());
  pub static ELEMENT_STYLES: RefCell<HashMap<String, gtk::CssProvider>> = RefCell::new(
    HashMap::new()
  );
}
