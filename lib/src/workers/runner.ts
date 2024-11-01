import { CFunction, dlopen, FFIType } from "bun:ffi";
import { randomUUID } from "crypto";
import { createFFI, toCString } from "../ffi";

declare var self: Worker;

self.onmessage = (e) => {
  const core = createFFI(e.data.lib_path);

  const id = toCString(e.data.id);
  const title = toCString(e.data.title);
  const icon = toCString(e.data.icon);

  core.initialize();

  const channel_ptr = core.create_window(title, icon, id);

  postMessage({
    e: "ready",
    channel_ptr,
  });

  core.run();

  postMessage({
    e: "close",
  });
};
