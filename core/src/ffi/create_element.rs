use std::os::raw::c_char;
use crate::utils::{ channel::ptr_to_channel_tx, pointer::ptr_to_str };

#[no_mangle]
// Function for creating an element
pub extern "C" fn create_element(
  tx_ptr: *mut async_channel::Sender<String>,
  tag: *const c_char,
  id: *const c_char,
  args: *const c_char
) -> () {
  // Convert the pointers to strings
  let tag = ptr_to_str(tag);
  let id = ptr_to_str(id);
  let args = ptr_to_str(args);

  // Get channel sender from the pointer
  let tx = ptr_to_channel_tx(tx_ptr).unwrap();

  // Send the message to the channel
  tx.try_send(format!("{};{};{};{}", "create_element", id, tag, args)).unwrap();

  ()
}
