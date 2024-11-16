use std::os::raw::c_char;
use crate::utils::{ channel::ptr_to_channel_tx, pointer::ptr_to_str };

#[no_mangle]
// Function for getting an attribute of an element
pub extern "C" fn get_attribute(
  tx_ptr: *mut async_channel::Sender<String>,
  id: *const c_char,
  tag: *const c_char,
  key: *const c_char,
  symbol_id: *const c_char
) -> () {
  // Convert the pointers to strings
  let id = ptr_to_str(id);
  let tag = ptr_to_str(tag);
  let key = ptr_to_str(key);
  let symbol_id = ptr_to_str(symbol_id);

  // Get channel sender from the pointer
  let tx = ptr_to_channel_tx(tx_ptr).unwrap();

  // Send the message to the channel
  tx.try_send(format!("{};{};{};{};{}", "get_attribute", id, tag, key, symbol_id)).unwrap();

  ()
}
