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

  // Initialize the application (FFI initialize function)
  core.initialize();

  // Create a new callback for the event listener + get_attribute function
  const cb = new JSCallback(
    (symbol_id: Pointer, iid: Pointer) => {
      // Decode the pointers to strings
      let dec_iid = new TextDecoder().decode(
        new Uint8Array(toArrayBuffer(iid))
      );
      let symb_id = new TextDecoder()
        .decode(new Uint8Array(toArrayBuffer(symbol_id)))
        // Get only the first 32 characters (symbol id), to fix junk messing with the data
        .substring(0, 32);

      // Check if the symbol id is special (for get_attribute function)
      if (symb_id.startsWith("!!")) {
        // Post the message to the main thread
        workerData.port.postMessage(
          // Decode the iid
          dec_iid.split("!!").slice(1).join("!!").split(";~;")[0]
        );

        // Free the main thread (blocked by the Atomics.wait)
        const i32 = new Int32Array(workerData.shared);
        Atomics.notify(i32, 0);

        return;
      }

      // Post the message to the main thread
      postMessage({
        e: "event",
        symbol_id: symb_id,
        // Get only the first 32 characters (iid), to fix junk messing with the data
        iid: dec_iid.substring(0, 32),
      });
    },
    {
      args: [FFIType.cstring, FFIType.cstring],
      returns: FFIType.void,
    }
  );

  // Create the window and get the channel pointer (FFI create_window function)
  const channel_ptr = core.create_window(title, icon, id, cb);

  // Post ready message to the main thread
  postMessage({
    e: "ready",
    channel_ptr,
  });

  // Run the main loop (FFI run function)
  core.run();

  // Post close message to the main thread (after all the windows are closed or main loop is stopped)
  postMessage({
    e: "close",
  });
};
