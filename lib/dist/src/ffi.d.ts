import { FFIType } from "bun:ffi";
export declare function createFFI(path: string): import("bun:ffi").ConvertFns<{
    initialize: {
        args: never[];
        returns: FFIType.bool;
    };
    run: {
        args: never[];
        returns: FFIType.void;
    };
    create_window: {
        args: (FFIType.cstring | FFIType.function)[];
        returns: FFIType.ptr;
    };
    create_element: {
        args: (FFIType.ptr | FFIType.cstring)[];
        returns: FFIType.void;
    };
    append_child: {
        args: (FFIType.ptr | FFIType.cstring)[];
        returns: FFIType.void;
    };
    set_attribute: {
        args: (FFIType.ptr | FFIType.cstring)[];
        returns: FFIType.void;
    };
    add_event_listener: {
        args: (FFIType.ptr | FFIType.cstring)[];
        returns: FFIType.void;
    };
    remove_element: {
        args: (FFIType.ptr | FFIType.cstring)[];
        returns: FFIType.void;
    };
    set_styles: {
        args: (FFIType.ptr | FFIType.cstring)[];
        returns: FFIType.void;
    };
    get_attribute: {
        args: (FFIType.ptr | FFIType.cstring)[];
        returns: FFIType.cstring;
    };
}>;
export declare function toCString(str: string): Uint8Array;
