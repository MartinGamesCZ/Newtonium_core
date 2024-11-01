import { FFIType, JSCallback, type Pointer } from "bun:ffi";
import type Window from "./window";
import { toCString } from "../ffi";

export type ElementTag = "view" | "text" | "button";

export default class Element {
  tagName: ElementTag;
  iid: string;

  private _window: Window;

  constructor(tag: ElementTag, iid: string, _window: Window) {
    this.tagName = tag;
    this.iid = iid;

    this._window = _window;
  }

  appendChild(child: Element) {
    this._window.core.append_child(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid),
      toCString(child.iid)
    );
  }

  setAttribute(key: string, value: string) {
    this._window.core.set_attribute(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid),
      toCString(this.tagName),
      toCString(key),
      toCString(value)
    );
  }

  addEventListener(event: string, listener: () => void) {
    const cb = new JSCallback(() => console.log("a"), {
      args: [],
      returns: FFIType.void,
    });

    this._window.core.attach_listener(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid),
      toCString(this.tagName),
      toCString(event),
      cb
    );

    // TODO: MEMORY LEAK - fix, cb is never freed
  }
}
