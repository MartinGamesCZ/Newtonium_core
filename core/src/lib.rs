mod dom;
mod elements;

use std::ffi::{ c_void, CStr };
use std::os::raw::c_char;
use std::sync::{ Arc, Mutex };
use dom::dom_create_element;
use elements::set_element_attribute;
use gdk::glib::translate::from_glib;
use gtk::ffi::gtk_init_check;
use once_cell::sync::Lazy;
use gtk::{ prelude::*, CssProvider, StyleProvider };
use gtk::{ Application, ApplicationWindow };
use serde_json::Value;
use std::sync::mpsc;
use tungstenite::{ connect, Message };
use std::collections::HashMap;
use lazy_static::lazy_static;

thread_local! {
  static WINDOWS: std::cell::RefCell<
    std::collections::HashMap<String, gtk::Window>
  > = std::cell::RefCell::new(std::collections::HashMap::new());
  static ELEMENTS: std::cell::RefCell<
    std::collections::HashMap<String, gtk::Widget>
  > = std::cell::RefCell::new(std::collections::HashMap::new());
  static ELEMENT_STYLES: std::cell::RefCell<
    std::collections::HashMap<String, CssProvider>
  > = std::cell::RefCell::new(std::collections::HashMap::new());
}

#[no_mangle]
pub extern "C" fn initialize() -> bool {
  let success: bool = unsafe {
    from_glib(gtk_init_check(std::ptr::null_mut(), std::ptr::null_mut()))
  };

  if !success {
    false;
  }

  unsafe {
    gtk::set_initialized();
  }

  true
}

#[no_mangle]
pub extern "C" fn create_window(
  title: *const c_char,
  icon: *const c_char,
  id: *const c_char,
  event_cb: extern "C" fn(*const c_char, *const c_char) -> ()
) -> *mut async_channel::Sender<String> {
  let title = (unsafe { CStr::from_ptr(title) }).to_str().unwrap();
  let icon = (unsafe { CStr::from_ptr(icon) }).to_str().unwrap();
  let win_id = (unsafe { CStr::from_ptr(id) }).to_str().unwrap();

  let window = gtk::Window::new(gtk::WindowType::Toplevel);

  window.set_icon_from_file(icon).unwrap_or(());
  window.set_default_size(480, 360);
  window.set_title(title);

  window.connect_delete_event(move |_, _| {
    WINDOWS.with(|windows| {
      let mut windows = windows.borrow_mut();

      windows.remove(win_id);

      if windows.is_empty() {
        gtk::main_quit();
      }
    });

    gdk::glib::Propagation::Proceed
  });

  let (tx, rx) = async_channel::unbounded::<String>();

  let bx = gtk::Box::new(gtk::Orientation::Vertical, 0);

  ELEMENTS.with(|elements| {
    elements.borrow_mut().insert("body".to_string(), bx.clone().upcast());
  });

  window.add(&bx);
  window.show_all();

  let c_window = window.clone();
  WINDOWS.with(|windows| {
    windows.borrow_mut().insert(win_id.to_string(), c_window);
  });

  let main_context = gtk::glib::MainContext::default();
  main_context.spawn_local(async move {
    while let Ok(msg) = rx.recv().await {
      let splt = msg.split(";").collect::<Vec<&str>>();

      match splt[0] {
        "create_element" => {
          let id = splt[1];

          let output: HashMap<String, Value> = serde_json::from_str(splt[3]).unwrap();

          let element = dom_create_element(splt[2], output);

          element.style_context().add_class(("iid_".to_string() + id).as_str());

          ELEMENTS.with(|elements| {
            elements.borrow_mut().insert(id.to_string(), element);
          });
        }
        "append_child" => {
          let parent_id = splt[1];
          let child_id = splt[2];

          let parent = ELEMENTS.with(|elements| {
            elements.borrow().get(parent_id).unwrap().clone()
          });

          let child = ELEMENTS.with(|elements| {
            elements.borrow().get(child_id).unwrap().clone()
          });

          let parent = parent.downcast::<gtk::Container>().unwrap();

          parent.add(&child);

          window.show_all();
        }
        "remove_element" => {
          let id = splt[1];

          ELEMENTS.with(|elements| {
            let element = elements.borrow().get(id).unwrap().clone();

            let parent = element.parent().unwrap().clone().downcast::<gtk::Container>().unwrap();

            parent.remove(&element);

            elements.borrow_mut().remove(id);
          });
        }
        "set_attribute" => {
          let id = splt[1];
          let tag = splt[2];
          let key = splt[3];
          let value = splt[4];

          let element = ELEMENTS.with(|elements| { elements.borrow().get(id).unwrap().clone() });

          set_element_attribute(tag, &element, key, value);
        }
        "add_event_listener" => {
          let id = splt[1].to_string();
          let event = splt[2];
          let symbol_id = splt[3].to_string();

          let element = ELEMENTS.with(|elements| {
            elements.borrow().get(id.as_str()).unwrap().clone()
          });

          element.connect(event, false, move |_| {
            event_cb(symbol_id.as_ptr() as *const i8, id.as_ptr() as *const i8);

            None
          });
        }
        "set_styles" => {
          let id = splt[1];
          let styles = splt[2].to_string();

          let element = ELEMENTS.with(|elements| { elements.borrow().get(id).unwrap().clone() });

          let old_provider = ELEMENT_STYLES.with(|element_styles| {
            let styles = element_styles.borrow_mut();

            let old_provider: Option<CssProvider> = styles.get(id).cloned();

            old_provider
          });

          if old_provider.is_some() {
            element.style_context().remove_provider(&old_provider.unwrap());
          }

          let provider = gtk::CssProvider::new();

          provider.load_from_data(styles.as_bytes()).unwrap();

          element.style_context().add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

          ELEMENT_STYLES.with(|styles| {
            styles.borrow_mut().insert(id.to_string(), provider.clone());
          });

          ();
        }
        _ => {
          println!("Unknown message: {}", msg);
        }
      }
    }
  });

  Box::into_raw(Box::new(tx))
}

#[no_mangle]
pub extern "C" fn run() -> () {
  gtk::main();

  ()
}

#[no_mangle]
pub extern "C" fn create_element(
  tx_ptr: *mut async_channel::Sender<String>,
  tag: *const c_char,
  id: *const c_char,
  args: *const c_char
) -> () {
  let tag = (unsafe { CStr::from_ptr(tag) }).to_str().unwrap();
  let id = (unsafe { CStr::from_ptr(id) }).to_str().unwrap();
  let args = (unsafe { CStr::from_ptr(args) }).to_str().unwrap();

  if tx_ptr.is_null() {
    println!("Pointer is null");
    return;
  }

  unsafe {
    let tx_ptr = tx_ptr as *const async_channel::Sender<String>;

    if tx_ptr.is_null() {
      println!("Window pointer is null");
      return;
    }

    match tx_ptr.as_ref() {
      Some(tx) => {
        tx.try_send(format!("{};{};{};{}", "create_element", id, tag, args)).unwrap();
        true;
      }
      None => {
        println!("Window is null");
        false;
      }
    }

    ();
  }
}

#[no_mangle]
pub extern "C" fn append_child(
  tx_ptr: *mut async_channel::Sender<String>,
  parent_id: *const c_char,
  child_id: *const c_char
) -> () {
  let parent_id = (unsafe { CStr::from_ptr(parent_id) }).to_str().unwrap();
  let child_id = (unsafe { CStr::from_ptr(child_id) }).to_str().unwrap();

  if tx_ptr.is_null() {
    println!("Pointer is null");
    return;
  }

  unsafe {
    let tx_ptr = tx_ptr as *const async_channel::Sender<String>;

    if tx_ptr.is_null() {
      println!("Window pointer is null");
      return;
    }

    match tx_ptr.as_ref() {
      Some(tx) => {
        tx.try_send(format!("{};{};{}", "append_child", parent_id, child_id)).unwrap();
        true;
      }
      None => {
        println!("Window is null");
        false;
      }
    }

    ();
  }
}

#[no_mangle]
pub extern "C" fn set_attribute(
  tx_ptr: *mut async_channel::Sender<String>,
  id: *const c_char,
  tag: *const c_char,
  key: *const c_char,
  value: *const c_char
) {
  let id = (unsafe { CStr::from_ptr(id) }).to_str().unwrap();
  let tag = (unsafe { CStr::from_ptr(tag) }).to_str().unwrap();
  let key = (unsafe { CStr::from_ptr(key) }).to_str().unwrap();
  let value = (unsafe { CStr::from_ptr(value) }).to_str().unwrap();

  if tx_ptr.is_null() {
    println!("Pointer is null");
    return;
  }

  unsafe {
    let tx_ptr = tx_ptr as *const async_channel::Sender<String>;

    if tx_ptr.is_null() {
      println!("Window pointer is null");
      return;
    }

    match tx_ptr.as_ref() {
      Some(tx) => {
        tx.try_send(format!("{};{};{};{};{}", "set_attribute", id, tag, key, value)).unwrap();
        true;
      }
      None => {
        println!("Window is null");
        false;
      }
    }

    ();
  }
}

#[no_mangle]
pub extern "C" fn add_event_listener(
  tx_ptr: *mut async_channel::Sender<String>,
  id: *const c_char,
  key: *const c_char,
  symbol_id: *const c_char
) {
  let id = (unsafe { CStr::from_ptr(id) }).to_str().unwrap();
  let key = (unsafe { CStr::from_ptr(key) }).to_str().unwrap();
  let symbol_id = (unsafe { CStr::from_ptr(symbol_id) }).to_str().unwrap();

  if tx_ptr.is_null() {
    println!("Pointer is null");
    return;
  }

  unsafe {
    let tx_ptr = tx_ptr as *const async_channel::Sender<String>;

    if tx_ptr.is_null() {
      println!("Window pointer is null");
      return;
    }

    match tx_ptr.as_ref() {
      Some(tx) => {
        tx.try_send(format!("{};{};{};{}", "add_event_listener", id, key, symbol_id)).unwrap();
        true;
      }
      None => {
        println!("Window is null");
        false;
      }
    }

    ();
  }
}

#[no_mangle]
pub extern "C" fn remove_element(tx_ptr: *mut async_channel::Sender<String>, id: *const c_char) {
  let id = (unsafe { CStr::from_ptr(id) }).to_str().unwrap();

  if tx_ptr.is_null() {
    println!("Pointer is null");
    return;
  }

  unsafe {
    let tx_ptr = tx_ptr as *const async_channel::Sender<String>;

    if tx_ptr.is_null() {
      println!("Window pointer is null");
      return;
    }

    match tx_ptr.as_ref() {
      Some(tx) => {
        tx.try_send(format!("{};{}", "remove_element", id)).unwrap();
        true;
      }
      None => {
        println!("Window is null");
        false;
      }
    }

    ();
  }
}

#[no_mangle]
pub extern "C" fn set_styles(
  tx_ptr: *mut async_channel::Sender<String>,
  id: *const c_char,
  styles: *const c_char
) {
  let id = (unsafe { CStr::from_ptr(id) }).to_str().unwrap();
  let styles = (unsafe { CStr::from_ptr(styles) }).to_str().unwrap();

  if tx_ptr.is_null() {
    println!("Pointer is null");
    return;
  }

  unsafe {
    let tx_ptr = tx_ptr as *const async_channel::Sender<String>;

    if tx_ptr.is_null() {
      println!("Window pointer is null");
      return;
    }

    match tx_ptr.as_ref() {
      Some(tx) => {
        tx.try_send(format!("{};{};{}", "set_styles", id, styles)).unwrap();
        true;
      }
      None => {
        println!("Window is null");
        false;
      }
    }

    ();
  }
}
