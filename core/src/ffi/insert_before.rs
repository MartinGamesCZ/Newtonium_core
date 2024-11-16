use std::os::raw::c_char;
use crate::utils::{ channel::ptr_to_channel_tx, pointer::ptr_to_str };

#[no_mangle]
// Function for inserting a child before a sibling
pub extern "C" fn insert_before(
  tx_ptr: *mut async_channel::Sender<String>,
  parent_id: *const c_char,
  child_id: *const c_char,
  sibling_id: *const c_char
) -> () {
  // Convert the pointers to strings
  let parent_id = ptr_to_str(parent_id);
  let child_id = ptr_to_str(child_id);
  let sibling_id = ptr_to_str(sibling_id);

  // Get channel sender from the pointer
  let tx = ptr_to_channel_tx(tx_ptr).unwrap();

  // Send the message to the channel
  tx.try_send(format!("{};{};{};{}", "insert_before", parent_id, child_id, sibling_id)).unwrap();

  ()
}
