use std::collections::HashMap;
use gtk::prelude::*;
use serde_json::Value;
use crate::{
  graphics::{
    canvas::{
      canvas_create_program,
      canvas_load,
      canvas_make_current,
      clear::canvas_graphics_clear,
      line::canvas_graphics_draw_line,
    },
    program::get_program,
  },
  units::length::length_to_px,
};

// Function for creating a new canvas element
// Canvas -> GtkGLArea
pub fn create_element_canvas(args: HashMap<String, Value>, window: gtk::Window) -> gtk::Widget {
  // Create a new GtkGLArea element
  let element = gtk::GLArea::new();

  // Set the initial attributes of the element
  args.iter().for_each(|(key, value)| {
    // Upcast the element to a GtkWidget
    let upcasted_element = element.upcast_ref::<gtk::Widget>();
    let str_value = value.as_str().unwrap();

    // Set the attribute of the element
    set_element_attribute_canvas(upcasted_element, key, str_value, &window);
  });

  element.set_hexpand(true);

  element.upcast()
}

// Function for setting the attribute of a canvas element
pub fn set_element_attribute_canvas(
  element: &gtk::Widget,
  key: &str,
  value: &str,
  window: &gtk::Window
) -> () {
  // Downcast the element to a GtkGLArea
  let downcasted_element = element.downcast_ref::<gtk::GLArea>().unwrap();

  // Get parent element
  let parent = downcasted_element.parent();

  // Set the attribute of the element
  match key {
    "@g_line" => {
      let split = value.split("/").collect::<Vec<&str>>();

      fn to_i32(value: &str) -> i32 {
        value.parse::<i32>().unwrap()
      }

      fn to_f32(value: &str) -> f32 {
        value.parse::<f32>().unwrap()
      }

      let sx = to_i32(split[0]);
      let sy = to_i32(split[1]);
      let sz = to_f32(split[2]);
      let ex = to_i32(split[3]);
      let ey = to_i32(split[4]);
      let ez = to_f32(split[5]);
      let cr = to_f32(split[6]);
      let cg = to_f32(split[7]);
      let cb = to_f32(split[8]);
      let ca = to_f32(split[9]);
      let lw = to_i32(split[10]);

      canvas_graphics_draw_line(
        sx,
        sy,
        sz,
        ex,
        ey,
        ez,
        lw,
        [cr / 256.0, cg / 256.0, cb / 256.0, ca / 256.0],
        downcasted_element
      );
    }
    "@g_load" => {
      canvas_load();
      canvas_create_program();
    }
    "@g_make_current" => {
      canvas_make_current(downcasted_element);
    }
    "@g_clear" => {
      canvas_graphics_clear();
    }
    "@g_update" => {
      downcasted_element.queue_draw();
    }
    "width" =>
      downcasted_element.set_width_request(
        length_to_px(
          value,
          match parent {
            Some(parent) => parent.width_request(),
            None => window.width_request(),
          },
          window
        )
      ),
    "height" =>
      downcasted_element.set_height_request(
        length_to_px(
          value,
          match parent {
            Some(parent) => parent.height_request(),
            None => window.height_request(),
          },
          window
        )
      ),

    // Panic if the attribute is unknown
    _ => panic!("Unknown attribute: {}", key),
  }

  ()
}

// Function for getting the attribute of a canvas element
pub fn get_element_attribute_canvas(element: &gtk::Widget, key: &str) -> String {
  // Downcast the element to a GtkGLArea
  let element = element.downcast_ref::<gtk::GLArea>().unwrap();

  // Get the attribute of the element
  let value = match key {
    "width" => element.width_request().to_string(),
    "height" => element.height_request().to_string(),

    // Panic if the attribute is unknown
    _ => panic!("Unknown attribute: {}", key),
  };

  value
}
