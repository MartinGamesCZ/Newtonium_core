use std::os::raw::c_char;

pub fn ptr_to_channel_tx(
  ptr: *mut async_channel::Sender<String>
) -> Option<async_channel::Sender<String>> {
  // Check if the pointer is null
  if ptr.is_null() {
    return None;
  }

  let tx = unsafe {
    // Convert the pointer to a channel sender
    let tx_ptr = ptr as *const async_channel::Sender<String>;

    // Check if the pointer is null
    if tx_ptr.is_null() {
      return None;
    }

    // Return the channel sender
    match tx_ptr.as_ref() {
      Some(tx) => { Some(tx.clone()) }
      None => { None }
    }
  };

  tx
}

// Function for generating a message frame pointer from a string
pub fn frame_to_ptr(data: &str) -> *const c_char {
  // Append a null terminator to the string
  let frame = data.to_owned() + "\0";

  // Convert the string to a raw pointer
  Box::into_raw(frame.into_boxed_str()) as *const c_char
}
