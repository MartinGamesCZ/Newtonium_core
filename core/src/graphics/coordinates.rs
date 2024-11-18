// Function to convert a coordinate (pixel) to an OpenGL coordinate (-1.0 to 1.0)
pub fn coordinate_to_gl(coordinate: i32, canvas_length: i32) -> f32 {
  ((coordinate as f32) / (canvas_length as f32)) * 2.0 - 1.0
}
