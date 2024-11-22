use crate::graphics::{ program::create_program, shader::create_shader };

pub fn g_create_program(id: &str, vertex_shader: &str, fragment_shader: &str) {
  let vertex_shader_id = id.to_owned() + "$vert_shader";
  let fragment_shader_id = id.to_owned() + "$frag_shader";

  create_shader(
    vertex_shader.replace("%~", ";").as_str(),
    vertex_shader_id.as_str(),
    gl::VERTEX_SHADER
  );
  create_shader(
    fragment_shader.replace("%~", ";").as_str(),
    fragment_shader_id.as_str(),
    gl::FRAGMENT_SHADER
  );
  create_program(vertex_shader_id.as_str(), fragment_shader_id.as_str(), id);
}
