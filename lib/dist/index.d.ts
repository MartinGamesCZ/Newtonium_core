export { default as Document } from "./dom/document";
export default class Window {
    id: string;
    worker: Worker;
    lib_path: string;
    constructor(id: string);
    setCustomBinaryPath(path: string): this;
    open(): void;
}
