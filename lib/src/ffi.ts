import { dlopen, FFIType } from "bun:ffi";

// Function for opening the core cdylib (with specified bindings)
export function createFFI(path: string) {
  // Open the cdylib and get the symbols
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
    insert_before: {
      args: [FFIType.ptr, FFIType.cstring, FFIType.cstring, FFIType.cstring],
      returns: FFIType.void,
    },
  });

  return symbols;
}

// Function for converting string to pointer (CString)
export function toCString(str: string) {
  // Convert string to Uint8Array and append null byte (string terminator)
  return new Uint8Array([...new TextEncoder().encode(str), 0]);
}
