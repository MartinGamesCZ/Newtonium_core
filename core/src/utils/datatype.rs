pub fn str_to_i32(value: &str) -> i32 {
  value.parse::<i32>().unwrap()
}

pub fn str_to_f32(value: &str) -> f32 {
  value.parse::<f32>().unwrap()
}
