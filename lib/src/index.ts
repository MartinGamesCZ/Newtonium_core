import type { Subprocess, SyncSubprocess } from "bun";
import { existsSync, fstatSync } from "fs";
import path from "path";
import { dlopen, FFIType, type Library } from "bun:ffi";
import { randomBytes } from "crypto";

export default class Window {
  qml: string;
  icon: string;
  appName: string;
  //private instanceSecret: string = randomBytes(32).toString("hex");

  worker: Worker;

  lib_path: string = path.join(
    import.meta.dirname,
    "lib/libnewtonium" + (process.platform == "win32" ? ".dll" : ".so")
  );

  constructor(qml: string, icon: string, appName: string) {
    this.qml = qml;
    this.icon = icon;
    this.appName = appName;

    this.worker = new Worker(path.join(import.meta.dirname, "worker.ts"));
  }

  setCustomBinaryPath(path: string) {
    this.lib_path = path;

    return this;
  }

  open() {
    this.worker.postMessage({
      lib_path: this.lib_path,
      qml: this.qml,
      icon: this.icon,
      appName: this.appName,
    });
  }
}
