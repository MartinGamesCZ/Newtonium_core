import Document from "./document";
export default class Window {
    worker_path: string;
    lib_path: string;
    private _worker;
    private _id;
    private _channel_ptr;
    core: import("bun:ffi").ConvertFns<{
        initialize: {
            args: never[];
            returns: import("bun:ffi").FFIType.bool;
        };
        run: {
            args: never[];
            returns: import("bun:ffi").FFIType.void;
        };
        create_window: {
            args: (import("bun:ffi").FFIType.cstring | import("bun:ffi").FFIType.function)[];
            returns: import("bun:ffi").FFIType.ptr;
        };
        create_element: {
            args: (import("bun:ffi").FFIType.ptr | import("bun:ffi").FFIType.cstring)[];
            returns: import("bun:ffi").FFIType.void;
        };
        append_child: {
            args: (import("bun:ffi").FFIType.ptr | import("bun:ffi").FFIType.cstring)[];
            returns: import("bun:ffi").FFIType.void;
        };
        set_attribute: {
            args: (import("bun:ffi").FFIType.ptr | import("bun:ffi").FFIType.cstring)[];
            returns: import("bun:ffi").FFIType.void;
        };
        add_event_listener: {
            args: (import("bun:ffi").FFIType.ptr | import("bun:ffi").FFIType.cstring)[];
            returns: import("bun:ffi").FFIType.void;
        };
        remove_element: {
            args: (import("bun:ffi").FFIType.ptr | import("bun:ffi").FFIType.cstring)[];
            returns: import("bun:ffi").FFIType.void;
        };
        set_styles: {
            args: (import("bun:ffi").FFIType.ptr | import("bun:ffi").FFIType.cstring)[];
            returns: import("bun:ffi").FFIType.void;
        };
    }>;
    title: string;
    icon: string;
    document: Document;
    listeners: {
        [key: string]: Function[];
    };
    element_listeners: {
        [key: string]: Function;
    };
    constructor(title: string, icon: string);
    run(): void;
    on(event: string, listener: Function): void;
    private _fireEvent;
    getChannelPtr(): number;
}
