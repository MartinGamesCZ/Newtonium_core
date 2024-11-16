use std::os::raw::c_char;
use crate::utils::{ channel::ptr_to_channel_tx, pointer::ptr_to_str };

#[no_mangle]
// Function for setting the styles of an element
pub extern "C" fn set_styles(
  tx_ptr: *mut async_channel::Sender<String>,
  id: *const c_char,
  styles: *const c_char
) -> () {
  // Convert the pointers to strings
  let id = ptr_to_str(id);
  let styles = ptr_to_str(styles);

  // Get channel sender from the pointer
  let tx = ptr_to_channel_tx(tx_ptr).unwrap();

  // Send the message to the channel
  tx.try_send(format!("{};{};{}", "set_styles", id, styles)).unwrap();

  ()
}
