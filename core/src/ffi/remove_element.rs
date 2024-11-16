use std::os::raw::c_char;
use crate::utils::{ channel::ptr_to_channel_tx, pointer::ptr_to_str };

#[no_mangle]
// Function for removing an element
pub extern "C" fn remove_element(
  tx_ptr: *mut async_channel::Sender<String>,
  id: *const c_char
) -> () {
  // Convert the pointers to strings
  let id = ptr_to_str(id);

  // Get channel sender from the pointer
  let tx = ptr_to_channel_tx(tx_ptr).unwrap();

  // Send the message to the channel
  tx.try_send(format!("{};{}", "remove_element", id)).unwrap();

  ()
}