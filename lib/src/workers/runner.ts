import { FFIType, JSCallback, toArrayBuffer, type Pointer } from "bun:ffi";
import { createFFI, toCString } from "../ffi";
import { workerData } from "worker_threads";

// Declare the worker type to prevent typescript errors
declare var self: Worker;

// Handle the message from the main thread
self.onmessage = (e) => {
  // Create a new ffi instance
  const core = createFFI(e.data.lib_path);

  // Convert the strings to pointers
  const id = toCString(e.data.id);
  const title = toCString(e.data.title);
  const icon = toCString(e.data.icon);
  const width = e.data.width;
  const height = e.data.height;

  // Initialize the application (FFI initialize function)
  core.initialize();

  // Create the callback for event listeners
  const event_listener_cb = new JSCallback(
    (symbol_id: Pointer) => {
      // Decode the symbol id
      let decoded_symbol_id = new TextDecoder().decode(
        new Uint8Array(toArrayBuffer(symbol_id))
      );

      // Post the message to the main thread
      postMessage({
        e: "event_listener",
        symbol_id: decoded_symbol_id,
      });
    },
    {
      args: [FFIType.cstring],
      returns: FFIType.void,
    }
  );

  // Get the pointers
  const elements_ptr = core.get_elements_pointer();
  const callbacks_ptr = core.get_callbacks_pointer();

  // Create the event listener callback (FFI create_callback function)
  core.create_callback(
    callbacks_ptr,
    event_listener_cb,
    toCString("event_listener")
  );

  // Create the window and get the channel pointer (FFI create_window function)
  const channel_ptr = core.create_window(title, icon, id, width, height);

  // Post ready message to the main thread
  postMessage({
    e: "ready",
    channel_ptr,
    elements_ptr,
    callbacks_ptr,
  });

  // Run the main loop (FFI run function)
  core.run();

  // Post close message to the main thread (after all the windows are closed or main loop is stopped)
  postMessage({
    e: "close",
  });
};
