import { dlopen, FFIType } from "bun:ffi";

declare var self: Worker;

self.onmessage = (e) => {
  const lib = dlopen(e.data.lib_path, {
    create_application: {
      args: [FFIType.cstring],
      returns: FFIType.ptr,
    },
    create_window: {
      args: [FFIType.ptr, FFIType.cstring],
      returns: FFIType.void,
    },
    modify_component: {
      args: [FFIType.cstring],
      returns: FFIType.void,
    },
  });

  let id = new TextEncoder().encode(e.data.id);

  id = new Uint8Array([...id, 0]);

  let app = null;

  try {
    app = lib.symbols.create_application(id);
  } catch (e) {
    console.log(e);
  }

  if (!app) throw new Error("No window");

  try {
    let title = new TextEncoder().encode("Hello, World!");
    title = new Uint8Array([...title, 0]);

    let o = lib.symbols.create_window(app, title);

    console.log(o);
  } catch (e) {
    console.log(e);
  }

  /*postMessage({
    e: "close",
  });*/
};
