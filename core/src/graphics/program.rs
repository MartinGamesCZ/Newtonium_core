use crate::GRAPHICS;

pub fn create_program(vertex_shader: &str, fragment_shader: &str, id: &str) -> () {
  let vertex_shader = crate::graphics::shader::get_shader(vertex_shader);
  let fragment_shader = crate::graphics::shader::get_shader(fragment_shader);

  GRAPHICS.with(|g| {
    let mut graphics = g.borrow_mut();

    let program = unsafe {
      let program: u32 = gl::CreateProgram();
      gl::AttachShader(program, vertex_shader);
      gl::AttachShader(program, fragment_shader);
      gl::LinkProgram(program);

      program
    };

    graphics.insert(id.to_string(), program);
  });
}

pub fn get_program(id: &str) -> u32 {
  GRAPHICS.with(|g| {
    let graphics = g.borrow();
    graphics.get(id).unwrap().to_owned()
  })
}
