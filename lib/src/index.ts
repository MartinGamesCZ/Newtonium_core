import path from "path";
import { WebSocketServer } from "ws";

export default class Window {
  id: string;
  //private instanceSecret: string = randomBytes(32).toString("hex");

  worker: Worker;

  lib_path: string = path.join(
    import.meta.dirname,
    "lib/libnewtonium" + (process.platform == "win32" ? ".dll" : ".so")
  );

  constructor(id: string) {
    this.id = id;

    this.worker = new Worker(path.join(import.meta.dirname, "worker"));

    this.worker.onmessage = (e) => {
      if (e.data.e == "close") {
        console.log("Received close message, terminating worker...");
        this.worker.terminate();
        return;
      }
    };

    Bun.serve({
      port: 8080,
      fetch(req, server) {
        server.upgrade(req, {
          data: {
            createdAt: Date.now(),
            channelId: new URL(req.url).searchParams.get("channelId"),
          },
        });
      },
      websocket: {
        open: (ws: any) => {
          console.log("WebSocket connection opened");
        },
        close: (ws: any) => {
          console.log("WebSocket connection closed");
        },
        message: (ws: any, msg: string) => {
          if (msg == "init~") {
            ws.send("new_title");
          }
        },
      },
    });
  }

  setCustomBinaryPath(path: string) {
    this.lib_path = path;

    return this;
  }

  open() {
    this.worker.postMessage({
      lib_path: this.lib_path,
      id: this.id,
    });
  }
}
