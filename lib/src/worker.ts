import { dlopen, FFIType } from "bun:ffi";

declare var self: Worker;

self.onmessage = (e) => {
  const lib = dlopen(e.data.lib_path, {
    open_window: {
      args: [FFIType.cstring, FFIType.cstring, FFIType.cstring],
      returns: FFIType.void,
    },
  });

  let title = new TextEncoder().encode(e.data.title);
  let url = new TextEncoder().encode(e.data.url);
  let instance_secret = new TextEncoder().encode(e.data.instance_secret);

  title = new Uint8Array([...title, 0]);
  url = new Uint8Array([...url, 0]);
  instance_secret = new Uint8Array([...instance_secret, 0]);

  lib.symbols.open_window(title, url, instance_secret);

  postMessage({
    e: close,
  });
};
