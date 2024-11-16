use std::os::raw::c_char;
use crate::utils::{ channel::ptr_to_channel_tx, pointer::ptr_to_str };

#[no_mangle]
// Function for adding an event listener
pub extern "C" fn add_event_listener(
  tx_ptr: *mut async_channel::Sender<String>,
  id: *const c_char,
  key: *const c_char,
  symbol_id: *const c_char
) -> () {
  // Convert the pointers to strings
  let id = ptr_to_str(id);
  let key = ptr_to_str(key);
  let symbol_id = ptr_to_str(symbol_id);

  // Get channel sender from the pointer
  let tx = ptr_to_channel_tx(tx_ptr).unwrap();

  // Send the message to the channel
  tx.try_send(format!("{};{};{};{}", "add_event_listener", id, key, symbol_id)).unwrap();

  ()
}
