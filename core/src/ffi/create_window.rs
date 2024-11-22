use std::os::raw::c_char;
use gtk::prelude::*;
use crate::{
  commands::{
    append_child::append_child,
    create_element::create_element,
    g_create_program::g_create_program,
    insert_before::insert_before,
    remove_element::remove_element,
    set_attribute::set_attribute,
    set_styles::set_styles,
  },
  utils::pointer::ptr_to_str,
  ELEMENTS,
  WINDOWS,
};

#[no_mangle]
// Function for creating a new window
pub extern "C" fn create_window(
  title: *const c_char,
  icon: *const c_char,
  id: *const c_char,
  width: i32,
  height: i32
) -> *mut async_channel::Sender<String> {
  // Convert the pointers to strings
  let title = ptr_to_str(title);
  let icon = ptr_to_str(icon);
  let win_id = ptr_to_str(id);

  // Create a new window
  let window = gtk::Window::new(gtk::WindowType::Toplevel);

  // Set the window's properties
  window.set_icon_from_file(icon).unwrap_or(());
  window.set_default_size(width, height);
  window.set_title(title.as_str());

  // Connect the delete event
  connect_delete_event(&window, &win_id);

  // Create a new channel for the window
  let (tx, rx) = async_channel::unbounded::<String>();

  // Create a new vertical box used as the body of the window
  let bx = gtk::Box::new(gtk::Orientation::Vertical, 0);

  // Add the vertical box to the elements list
  ELEMENTS.with(|elements| {
    // Upcast the vertical box to a widget
    let upcased_element = bx.clone().upcast();

    // Insert the vertical box into the elements list
    elements.lock().unwrap().insert("body".to_string(), upcased_element);
  });

  // Add the vertical box to the window
  window.add(&bx);
  window.show_all();

  // Clone the window and add it to the windows list
  let c_window = window.clone();
  WINDOWS.with(|windows| {
    windows.borrow_mut().insert(win_id.to_string(), c_window);
  });

  // Spawn a new thread for the window
  spawn_window_thread(rx, window);

  // Return the channel sender pointer
  Box::into_raw(Box::new(tx))
}

// Function for connecting the delete event of a window
fn connect_delete_event(window: &gtk::Window, id: &str) -> () {
  // Convert the ID to a string
  let id = id.to_string();

  // Connect the delete event
  window.connect_delete_event(move |_, _| {
    WINDOWS.with(|windows| {
      // Get the windows list
      let mut windows = windows.borrow_mut();

      // Remove the window from the windows list
      windows.remove(&id);

      // If the windows list is empty, quit the GTK application
      if windows.is_empty() {
        gtk::main_quit();
      }
    });

    gdk::glib::Propagation::Proceed
  });

  ()
}

// Function for spawning a new thread for a window
fn spawn_window_thread(rx: async_channel::Receiver<String>, window: gtk::Window) -> () {
  // Create new main context
  let main_context = gtk::glib::MainContext::default();

  // Spawn a new local task
  main_context.spawn_local(async move {
    // Loop through the messages
    while let Ok(msg) = rx.recv().await {
      // Split the message
      let splt = msg.split(";").collect::<Vec<&str>>();

      // Match the first part of the message (command) and execute the corresponding function
      match splt[0] {
        "create_element" => create_element(splt[1], splt[2], splt[3], &window),
        "append_child" => append_child(splt[1], splt[2], &window),
        "remove_element" => remove_element(splt[1]),
        "set_attribute" => set_attribute(splt[1], splt[2], splt[3], splt[4], &window),
        "set_styles" => set_styles(splt[1], splt[2]),
        "insert_before" => insert_before(splt[1], splt[2], splt[3], &window),
        "g_create_program" => g_create_program(splt[1], splt[2], splt[3]),

        // If the message is unknown, panic
        _ => panic!("Unknown message: {}", msg),
      }
    }
  });

  ()
}
