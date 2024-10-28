import { dlopen, FFIType } from "bun:ffi";

declare var self: Worker;

self.onmessage = (e) => {
  const lib = dlopen(e.data.lib_path, {
    open_window: {
      args: [FFIType.cstring, FFIType.cstring, FFIType.cstring],
      returns: FFIType.void,
    },
  });

  let qml = new TextEncoder().encode(e.data.qml);
  let icon = new TextEncoder().encode(e.data.icon);
  let appName = new TextEncoder().encode(e.data.appName);

  qml = new Uint8Array([...qml, 0]);
  icon = new Uint8Array([...icon, 0]);
  appName = new Uint8Array([...appName, 0]);

  lib.symbols.open_window(qml, icon, appName);

  postMessage({
    e: close,
  });
};

const p = import.meta.filename;
export default p;
