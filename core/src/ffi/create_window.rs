use std::os::raw::c_char;
use gtk::prelude::*;
use crate::{
  commands::{
    add_event_listener::add_event_listener,
    append_child::append_child,
    create_element::create_element,
    get_attribute::get_attribute,
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
  event_cb: extern "C" fn(*const c_char, *const c_char) -> ()
) -> *mut async_channel::Sender<String> {
  // Convert the pointers to strings
  let title = ptr_to_str(title);
  let icon = ptr_to_str(icon);
  let win_id = ptr_to_str(id);

  // Create a new window
  let window = gtk::Window::new(gtk::WindowType::Toplevel);

  // Set the windows' properties
  window.set_icon_from_file(icon).unwrap_or(());
  window.set_default_size(480, 360);
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

    elements.borrow_mut().insert("body".to_string(), upcased_element);
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
  spawn_window_thread(rx, event_cb, window);

  Box::into_raw(Box::new(tx))
}

// Function for connecting the delete event of a window
fn connect_delete_event(window: &gtk::Window, id: &str) -> () {
  let id = id.to_string();

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
fn spawn_window_thread(
  rx: async_channel::Receiver<String>,
  event_cb: extern "C" fn(*const c_char, *const c_char) -> (),
  window: gtk::Window
) -> () {
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
        "create_element" => create_element(splt[1], splt[2], splt[3]),
        "append_child" => append_child(splt[1], splt[2], &window),
        "remove_element" => remove_element(splt[1]),
        "set_attribute" => set_attribute(splt[1], splt[2], splt[3], splt[4]),
        "get_attribute" => get_attribute(splt[1], splt[2], splt[3], splt[4], event_cb),
        "add_event_listener" => add_event_listener(splt[1], splt[2], splt[3], event_cb),
        "set_styles" => set_styles(splt[1], splt[2]),
        "insert_before" => insert_before(splt[1], splt[2], splt[3], &window),

        // If the message is unknown, panic
        _ => panic!("Unknown message: {}", msg),
      }
    }
  });

  ()
}
