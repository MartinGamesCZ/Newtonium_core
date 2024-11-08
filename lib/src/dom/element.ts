import { FFIType, JSCallback, type Pointer } from "bun:ffi";
import type Window from "./window";
import { toCString } from "../ffi";
import { randomId } from "../utils/id";
import cssTransformer from "../utils/css_transformer";

export type ElementTag = "view" | "text" | "button" | "input";

export default class Element {
  tagName: ElementTag;
  iid: string;
  _symbols: Map<string, string[]> = new Map();

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

      const transformed = cssTransformer(this.iid, this.style);

      this._window.core.set_styles(
        this._window.getChannelPtr() as Pointer,
        toCString(this.iid),
        toCString(transformed)
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

  getAttribute(key: string) {
    const sid = randomId();

    return new Promise((r) => {
      this._window.element_listeners[sid] = (val: string) => {
        r(val);
      };

      this._window.core.get_attribute(
        this._window.getChannelPtr() as Pointer,
        toCString(this.iid),
        toCString(this.tagName),
        toCString(key),
        toCString(sid)
      );
    });
  }

  addEventListener(event: string, listener: () => void) {
    const symbol_id = randomId();

    this._window.element_listeners[symbol_id] = listener;

    this._symbols.set(event, [...(this._symbols.get(event) || []), symbol_id]);

    this._window.core.add_event_listener(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid),
      toCString(event),
      toCString(symbol_id)
    );

    return symbol_id;
  }

  removeEventListener(event: string) {
    const symbol_ids = this._symbols.get(event);

    if (!symbol_ids) return;

    symbol_ids.forEach((symbol_id) => {
      this._window.element_listeners[symbol_id] = () => {};
      delete this._window.element_listeners[symbol_id];
    });
  }

  remove() {
    this._window.core.remove_element(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid)
    );

    this._window.document.deleteElement(this.iid);
  }
}
