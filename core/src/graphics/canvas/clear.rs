// Function to clear the canvas
pub fn canvas_graphics_clear() {
  // Clear the canvas
  unsafe {
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
  }
}
