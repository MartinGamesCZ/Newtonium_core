import path from "path";
import Document from "./document";
import { randomId } from "../utils/id";
import { createFFI } from "../ffi";
import { MessageChannel, MessagePort, Worker } from "worker_threads";

export default class Window {
  worker_path: string = path.join(import.meta.dirname, "../workers/runner");
  lib_path: string = path.join(
    import.meta.dirname,
    "../lib/libnewtonium" + (process.platform == "win32" ? ".dll" : ".so")
  );

  private _worker: Worker;
  private _id: string;
  private _channel_ptr: number = -1;
  core = createFFI(this.lib_path);
  title: string;
  icon: string;
  document: Document;
  localPort: MessagePort;
  shared: SharedArrayBuffer;

  listeners: {
    [key: string]: Function[];
  } = {
    ready: [],
  };

  element_listeners: {
    [key: string]: Function;
  } = {};

  constructor(title: string, icon: string) {
    const ports = new MessageChannel();
    this.localPort = ports.port1;

    this.shared = new SharedArrayBuffer(4);

    this._worker = new Worker(this.worker_path, {
      workerData: {
        port: ports.port2,
        shared: this.shared,
      },
      transferList: [ports.port2],
    });
    this._id = randomId();
    this.title = title;
    this.icon = icon;
    this.document = new Document(this);
  }

  run() {
    this._worker.postMessage({
      lib_path: this.lib_path,
      id: this._id,
      title: this.title,
      icon: this.icon,
    });

    this._worker.on("message", (e) => {
      if (e.e === "close") this._worker.terminate();
      if (e.e === "ready") {
        this._channel_ptr = e.channel_ptr;
        this._fireEvent("ready");
      }
      if (e.e === "event") {
        const listener =
          this.element_listeners[
            e.symbol_id.startsWith("!!") ? e.iid.split("!!")[0] : e.symbol_id
          ];

        if (!listener) return;

        if (e.symbol_id.startsWith("!!")) {
          listener(e.iid.split("!!").slice(1).join("!!").split(";~;")[0]);

          delete this.element_listeners[e.symbol_id];

          return;
        }

        listener();
      }
    });
  }

  on(event: string, listener: Function) {
    if (!this.listeners[event]) this.listeners[event] = [];

    this.listeners[event].push(listener);
  }

  private _fireEvent(event: string, ...args: any[]) {
    for (const listener of this.listeners[event]) {
      listener(...args);
    }
  }

  getChannelPtr() {
    if (this._channel_ptr === -1) throw new Error("Window is not ready yet");

    return this._channel_ptr;
  }
}
