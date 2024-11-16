import path from "path";
import Document from "./document";
import { randomId } from "../utils/id";
import { createFFI } from "../ffi";
import { MessageChannel, MessagePort, Worker } from "worker_threads";

// Window class, represents a window (virtually)
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
    // Create a new message channel
    const ports = new MessageChannel();
    this.localPort = ports.port1;

    // Create a new shared array buffer (used for blocking)
    this.shared = new SharedArrayBuffer(4);

    // Create a new thread for running gtk
    this._worker = new Worker(this.worker_path, {
      workerData: {
        port: ports.port2,
        shared: this.shared,
      },
      transferList: [ports.port2],
    });

    // Save to properties
    this._id = randomId();
    this.title = title;
    this.icon = icon;
    this.document = new Document(this);
  }

  // Function to run the application
  run() {
    // Post a message to the worker
    this._worker.postMessage({
      lib_path: this.lib_path,
      id: this._id,
      title: this.title,
      icon: this.icon,
    });

    // Listen for messages
    this._worker.on("message", (e) => {
      // If the message is close, terminate the worker
      if (e.e === "close") this._worker.terminate();

      // If the message is ready, save the channel pointer and fire the ready event
      if (e.e === "ready") {
        this._channel_ptr = e.channel_ptr;
        this._fireEvent("ready");
      }

      // If the message is event callback, fire the callback
      if (e.e === "event") {
        // Get the listener for the event
        const listener =
          this.element_listeners[
            e.symbol_id.startsWith("!!") ? e.iid.split("!!")[0] : e.symbol_id
          ];

        // If there is no listener, return
        if (!listener) return;

        // If the event is special (like get_attributes callback), call the listener with the data
        if (e.symbol_id.startsWith("!!")) {
          // Call the listener with the data (parse the data from the iid)
          listener(e.iid.split("!!").slice(1).join("!!").split(";~;")[0]);

          // Delete the listener, as it is a one-time listener
          delete this.element_listeners[e.symbol_id];

          return;
        }

        // Call the listener, no data to pass
        listener();
      }
    });
  }

  // Function to add an event listener
  on(event: string, listener: Function) {
    // If there is no listeners array for the event, create one
    if (!this.listeners[event]) this.listeners[event] = [];

    // Push the listener to the listeners array
    this.listeners[event].push(listener);
  }

  // Function to fire an event
  private _fireEvent(event: string, ...args: any[]) {
    // For each listener in the event listeners array, call the listener with the arguments
    for (const listener of this.listeners[event]) {
      listener(...args);
    }
  }

  // Function to get the channel pointer
  getChannelPtr() {
    // If the channel pointer is not set, throw an error
    // Should not happen when used correctly, as the channel pointer is set when the window is ready
    if (this._channel_ptr === -1) throw new Error("Window is not ready yet");

    return this._channel_ptr;
  }
}
