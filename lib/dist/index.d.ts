import type { Subprocess } from "bun";
export default class Window {
    title: string;
    url: string;
    process: Subprocess<"pipe", "pipe", "pipe"> | null;
    binary_path: string;
    event_listeners: {
        [key: string]: Function[];
    };
    constructor(title: string, url: string);
    setCustomBinaryPath(path: string): this;
    run_binary(): Promise<void>;
    send_command(cmd: string): Promise<void>;
    on(name: string, fn: Function): void;
    _fire_event(name: string, ...args: any[]): void;
    open(): void;
    ipc: {
        listeners: Array<Function>;
        onMessage: (fn: Function) => void;
        send: (msg: string) => void;
        _fire_recv: (msg: string) => void;
    };
}
