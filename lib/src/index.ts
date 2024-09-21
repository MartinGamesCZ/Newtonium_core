import type { Subprocess, SyncSubprocess } from "bun";
import { existsSync, fstatSync } from "fs";
import path from "path";

export default class Window {
  title: string;
  url: string;

  process: Subprocess<"pipe", "pipe", "pipe"> | null = null;

  binary_path: string = path.join(
    import.meta.dirname,
    "binaries/newtonium" + (process.platform == "win32" ? ".exe" : ""),
  );

  event_listeners: {
    [key: string]: Function[];
  } = {};

  constructor(title: string, url: string) {
    this.title = title;
    this.url = url;

    this.on("ready", () => {
      this.send_command("window_title_set::" + this.title);
      this.send_command("window_url_set::" + this.url);
    });

    this.on("ipc", (data: any) => {
      console.log("NEWTONIUM::CORE::IPC", data[1].replace(/^"|"$/g, ""));

      this.ipc._fire_recv(data[1].replace(/^"|"$/g, ""));
    });
  }

  setCustomBinaryPath(path: string) {
    this.binary_path = path;

    return this;
  }

  async run_binary() {
    let process = Bun.spawn({
      cmd: [this.binary_path],
      stdio: ["pipe", "pipe", "pipe"],
      stdin: "pipe",
    });

    this.process = process;

    process.stdout.pipeTo(
      new WritableStream({
        write: (chunk) => {
          const data = new TextDecoder().decode(chunk);

          const split = data.split("::").map((a) => a.trim());

          if (split[0] == "event") {
            this._fire_event(split[1], split.slice(2));
          }
        },
      }),
    );
  }

  async send_command(cmd: string) {
    console.log("NEWTONIUM::CORE::SEND_COMMAND", cmd);

    if (
      this.process &&
      this.process.stdin &&
      typeof this.process.stdin != "number"
    )
      this.process.stdin.write(new TextEncoder().encode(cmd + "\n"));
  }

  on(name: string, fn: Function) {
    if (!this.event_listeners[name]) this.event_listeners[name] = [];

    this.event_listeners[name].push(fn);
  }

  _fire_event(name: string, ...args: any[]) {
    console.log("NEWTONIUM::CORE::FIRE_EVENT", name);

    if (this.event_listeners[name]) {
      this.event_listeners[name].forEach((fn) => {
        fn(...args);
      });
    }
  }

  open() {
    this.run_binary();

    if (this.process) {
      if (this.process.stdin && typeof this.process.stdin != "number")
        this.process.stdin.write(new TextEncoder().encode(this.title + "\n"));
    }
  }

  ipc = {
    listeners: [] as Array<Function>,

    onMessage: (fn: Function) => {
      this.ipc.listeners.push(fn);
    },

    send: (msg: string) => {
      this.send_command("ipc_message::" + msg);
    },

    _fire_recv: (msg: string) => {
      this.ipc.listeners.forEach((fn) => {
        fn(msg);
      });
    },
  };
}
