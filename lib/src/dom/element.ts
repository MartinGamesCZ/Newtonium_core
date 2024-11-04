import { FFIType, JSCallback, type Pointer } from "bun:ffi";
import type Window from "./window";
import { toCString } from "../ffi";
import { randomId } from "../utils/id";
import cssTransformer from "../utils/css_transformer";

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

  style: {
    [key: string]: any;
    setProperty: (key: string, value: string) => void;
  } = {
    setProperty: (key: string, value: string) => {
      this.style[key] = value;

      this._window.core.set_styles(
        this._window.getChannelPtr() as Pointer,
        toCString(this.iid),
        toCString(cssTransformer(this.iid, this.style))
      );
    },
  };

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
