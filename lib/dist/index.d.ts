export default class Window {
    qml: string;
    icon: string;
    appName: string;
    worker: Worker;
    lib_path: string;
    constructor(qml: string, icon: string, appName: string);
    setCustomBinaryPath(path: string): this;
    open(): void;
}
