import { type Library } from "bun:ffi";
export default class Window {
    title: string;
    url: string;
    lib: Library<any>;
    lib_path: string;
    constructor(title: string, url: string);
    setCustomBinaryPath(path: string): this;
    open(): void;
}
