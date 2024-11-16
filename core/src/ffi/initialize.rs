use std::{ env, ptr::null_mut };

use gdk::glib::translate::from_glib;
use gtk::ffi::gtk_init_check;

#[no_mangle]
// Function for initializing the GTK application
pub extern "C" fn initialize() -> bool {
  // Set the env variables
  env::set_var("GTK_CSD", "0"); // -> Switch off client side decorations

  // Initialize the GTK application
  let success: bool = unsafe { from_glib(gtk_init_check(null_mut(), null_mut())) };

  // Set the initialized flag
  if success {
    unsafe {
      gtk::set_initialized();
    }
  }

  success
}
