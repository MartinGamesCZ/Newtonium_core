import type { Subprocess, SyncSubprocess } from "bun";
import { existsSync, fstatSync } from "fs";
import path from "path";
import { dlopen, FFIType, type Library } from "bun:ffi";
import { randomBytes } from "crypto";

export default class Window {
  title: string;
  url: string;
  private instanceSecret: string = randomBytes(32).toString("hex");

  worker: Worker;

  lib_path: string = path.join(
    import.meta.dirname,
    "lib/libnewtonium" + (process.platform == "win32" ? ".dll" : ".so")
  );

  constructor(title: string, url: string) {
    this.title = title;
    this.url = url;

    this.worker = new Worker(path.join(import.meta.dirname, "worker.ts"));
  }

  setCustomBinaryPath(path: string) {
    this.lib_path = path;

    return this;
  }

  open() {
    this.worker.postMessage({
      lib_path: this.lib_path,
      title: this.title,
      url: this.url,
      instance_secret: this.instanceSecret,
    });

    console.log("Sent message to worker!");
  }
}
