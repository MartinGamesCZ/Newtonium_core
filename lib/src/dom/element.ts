import { FFIType, JSCallback, type Pointer } from "bun:ffi";
import type Window from "./window";
import { toCString } from "../ffi";
import { randomId } from "../utils/id";

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
    const symbol_id = randomId();

    this._window.element_listeners[symbol_id] = listener;

    this._window.core.add_event_listener(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid),
      toCString(event),
      toCString(symbol_id)
    );
  }

  remove() {
    this._window.core.remove_element(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid)
    );

    this._window.document.deleteElement(this.iid);
  }
}
