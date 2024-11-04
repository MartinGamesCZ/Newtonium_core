import path from "path";
import Document from "./document";
import { randomId } from "../utils/id";
import { createFFI } from "../ffi";

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

  listeners: {
    [key: string]: Function[];
  } = {
    ready: [],
  };

  element_listeners: {
    [key: string]: Function;
  } = {};

  constructor(title: string, icon: string) {
    this._worker = new Worker(this.worker_path);
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

    this._worker.onmessage = (e) => {
      if (e.data.e === "close") this._worker.terminate();
      if (e.data.e === "ready") {
        this._channel_ptr = e.data.channel_ptr;
        this._fireEvent("ready");
      }
      if (e.data.e === "event") {
        const listener = this.element_listeners[e.data.symbol_id];

        if (!listener) return;

        listener();
      }
    };
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
