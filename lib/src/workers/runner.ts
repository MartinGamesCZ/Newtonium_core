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
      let dec_iid = new TextDecoder().decode(
        new Uint8Array(toArrayBuffer(iid))
      );
      let symb_id = new TextDecoder()
        .decode(new Uint8Array(toArrayBuffer(symbol_id)))
        .substring(0, 32);

      postMessage({
        e: "event",
        symbol_id: symb_id,
        iid: symb_id.startsWith("!!") ? dec_iid : dec_iid.substring(0, 32),
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
