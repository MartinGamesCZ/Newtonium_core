import {
  CFunction,
  dlopen,
  FFIType,
  JSCallback,
  read,
  toArrayBuffer,
  type Pointer,
} from "bun:ffi";
import { randomUUID } from "crypto";
import { createFFI, toCString } from "../ffi";

declare var self: Worker;

self.onmessage = (e) => {
  const core = createFFI(e.data.lib_path);

  const id = toCString(e.data.id);
  const title = toCString(e.data.title);
  const icon = toCString(e.data.icon);

  core.initialize();

  const cb = new JSCallback(
    (symbol_id: Pointer, iid: Pointer) => {
      postMessage({
        e: "event",
        symbol_id: new TextDecoder()
          .decode(new Uint8Array(toArrayBuffer(symbol_id)))
          .substring(0, 32),
        iid: new TextDecoder()
          .decode(new Uint8Array(toArrayBuffer(iid)))
          .substring(0, 32),
      });
    },
    {
      args: [FFIType.cstring, FFIType.cstring],
      returns: FFIType.void,
    }
  );

  const channel_ptr = core.create_window(title, icon, id, cb);

  postMessage({
    e: "ready",
    channel_ptr,
  });

  core.run();

  postMessage({
    e: "close",
  });
};
