import { dlopen, FFIType } from "bun:ffi";

export function createFFI(path: string) {
  const { symbols } = dlopen(path, {
    initialize: {
      args: [],
      returns: FFIType.bool,
    },
    run: {
      args: [],
      returns: FFIType.void,
    },
    create_window: {
      args: [
        FFIType.cstring,
        FFIType.cstring,
        FFIType.cstring,
        FFIType.function,
      ],
      returns: FFIType.ptr,
    },
    create_element: {
      args: [FFIType.ptr, FFIType.cstring, FFIType.cstring, FFIType.cstring],
      returns: FFIType.void,
    },
    append_child: {
      args: [FFIType.ptr, FFIType.cstring, FFIType.ptr],
      returns: FFIType.void,
    },
    set_attribute: {
      args: [
        FFIType.ptr,
        FFIType.cstring,
        FFIType.cstring,
        FFIType.cstring,
        FFIType.cstring,
      ],
      returns: FFIType.void,
    },
    add_event_listener: {
      args: [FFIType.ptr, FFIType.cstring, FFIType.cstring, FFIType.cstring],
      returns: FFIType.void,
    },
    remove_element: {
      args: [FFIType.ptr, FFIType.cstring],
      returns: FFIType.void,
    },
    set_styles: {
      args: [FFIType.ptr, FFIType.cstring, FFIType.cstring],
      returns: FFIType.void,
    },
    get_attribute: {
      args: [
        FFIType.ptr,
        FFIType.cstring,
        FFIType.cstring,
        FFIType.cstring,
        FFIType.cstring,
      ],
      returns: FFIType.cstring,
    },
  });

  return symbols;
}

export function toCString(str: string) {
  return new Uint8Array([...new TextEncoder().encode(str), 0]);
}
