use gtk::prelude::*;

// Function to convert length into pixels
// Used to support alternative units like vw, rem, %, ...
pub fn length_to_px(length: &str, container_length: i32, window: &gtk::Window) -> i32 {
  // Parse the length value and unit
  let value_end = length
    .chars()
    .position(|c| !c.is_numeric() && c != '.')
    .unwrap_or(length.len());

  let value: f32 = length[..value_end].parse().unwrap_or(0.0);
  let unit = &length[value_end..];

  // Convert to pixels based on unit
  match unit {
    "px" => value as i32,
    "%" => container_length * ((value / 100.0) as i32),
    "rem" => (value * 16.0) as i32,
    "em" => (value * 16.0) as i32,
    "vh" => ((value * (window.size().1 as f32)) / 100.0) as i32,
    "vw" => ((value * (window.size().0 as f32)) / 100.0) as i32,
    _ => value as i32,
  }
}
