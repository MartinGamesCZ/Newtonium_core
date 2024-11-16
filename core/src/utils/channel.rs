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
