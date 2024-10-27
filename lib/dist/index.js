import { existsSync, fstatSync } from "fs";
import path from "path";
import { dlopen, FFIType } from "bun:ffi";
export default class Window {
    title;
    url;
    lib;
    lib_path = path.join(import.meta.dirname, "lib/libnewtonium" + (process.platform == "win32" ? ".dll" : ".so"));
    constructor(title, url) {
        this.title = title;
        this.url = url;
        this.lib = dlopen(this.lib_path, {
            open_window: {
                args: [FFIType.cstring, FFIType.cstring],
                returns: FFIType.void,
            },
        });
    }
    setCustomBinaryPath(path) {
        this.lib_path = path;
        this.lib = dlopen(this.lib_path, {
            open_window: {
                args: [FFIType.cstring, FFIType.cstring],
                returns: FFIType.void,
            },
        });
        return this;
    }
    open() {
        let title = new TextEncoder().encode(this.title);
        let url = new TextEncoder().encode(this.url);
        title = new Uint8Array([...title, 0]);
        url = new Uint8Array([...url, 0]);
        this.lib.symbols.open_window(title, url);
    }
}
