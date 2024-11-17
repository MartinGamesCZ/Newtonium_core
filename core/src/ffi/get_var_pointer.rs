use std::collections::HashMap;
use std::sync::Mutex;
use crate::{ CALLBACKS, ELEMENTS };

#[no_mangle]
// Function for getting the elements pointer
extern "C" fn get_elements_pointer() -> *mut Mutex<HashMap<String, gtk::Widget>> {
  ELEMENTS.with(|elements| elements as *const _ as *mut _)
}

#[no_mangle]
// Function for getting the callbacks pointer
extern "C" fn get_callbacks_pointer() -> *mut Mutex<HashMap<String, extern "C" fn() -> ()>> {
  CALLBACKS.with(|callbacks| callbacks as *const _ as *mut _)
}
