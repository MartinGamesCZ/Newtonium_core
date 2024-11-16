use std::os::raw::c_char;
use crate::utils::{ channel::ptr_to_channel_tx, pointer::ptr_to_str };

#[no_mangle]
// Function for appending a child to a parent
pub extern "C" fn append_child(
  tx_ptr: *mut async_channel::Sender<String>,
  parent_id: *const c_char,
  child_id: *const c_char
) -> () {
  // Convert the pointers to strings
  let parent_id = ptr_to_str(parent_id);
  let child_id = ptr_to_str(child_id);

  // Get channel sender from the pointer
  let tx = ptr_to_channel_tx(tx_ptr).unwrap();

  // Send the message to the channel
  tx.try_send(format!("{};{};{}", "append_child", parent_id, child_id)).unwrap();

  ()
}
