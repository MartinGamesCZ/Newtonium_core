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
      rectangle::canvas_graphics_draw_rectangle,
      vertices::canvas_graphics_draw_vertices,
    },
    program::get_program,
  },
  units::length::length_to_px,
  utils::datatype::{ str_to_f32, str_to_i32 },
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

      let sx = str_to_i32(split[0]);
      let sy = str_to_i32(split[1]);
      let sz = str_to_f32(split[2]);
      let ex = str_to_i32(split[3]);
      let ey = str_to_i32(split[4]);
      let ez = str_to_f32(split[5]);
      let cr = str_to_f32(split[6]);
      let cg = str_to_f32(split[7]);
      let cb = str_to_f32(split[8]);
      let ca = str_to_f32(split[9]);
      let lw = str_to_i32(split[10]);

      let program = get_program("@g_line");

      canvas_graphics_draw_line(
        program,
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
    "@g_rectangle" => {
      let split = value.split("/").collect::<Vec<&str>>();

      let sx = str_to_i32(split[0]);
      let sy = str_to_i32(split[1]);
      let sz = str_to_f32(split[2]);
      let ex = str_to_i32(split[3]);
      let ey = str_to_i32(split[4]);
      let ez = str_to_f32(split[5]);
      let cr = str_to_f32(split[6]);
      let cg = str_to_f32(split[7]);
      let cb = str_to_f32(split[8]);
      let ca = str_to_f32(split[9]);

      let program = get_program("@g_line");

      canvas_graphics_draw_rectangle(
        program,
        sx,
        sy,
        sz,
        ex,
        ey,
        ez,
        [cr / 256.0, cg / 256.0, cb / 256.0, ca / 256.0],
        downcasted_element
      );
    }
    "@g_vertices" => {
      let split = value.split("/").collect::<Vec<&str>>();

      let vertices = split[1].split("!").collect::<Vec<&str>>();

      let vertices = vertices
        .iter()
        .map(|vertex| {
          let vertex = vertex.split(",").collect::<Vec<&str>>();
          let vertex = vertex
            .iter()
            .map(|v| v.parse::<f32>().unwrap())
            .collect::<Vec<f32>>();
          vertex
        })
        .collect::<Vec<Vec<f32>>>();

      let cr = str_to_f32(split[2]);
      let cg = str_to_f32(split[3]);
      let cb = str_to_f32(split[4]);
      let ca = str_to_f32(split[5]);

      let program = get_program(split[0]);

      canvas_graphics_draw_vertices(
        program,
        vertices,
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
