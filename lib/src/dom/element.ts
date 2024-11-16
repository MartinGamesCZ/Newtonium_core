import { FFIType, JSCallback, type Pointer } from "bun:ffi";
import type Window from "./window";
import { toCString } from "../ffi";
import { randomId } from "../utils/id";
import cssTransformer from "../utils/css_transformer";
import { receiveMessageOnPort } from "worker_threads";

// Element tags available to use
export type ElementTag = "view" | "text" | "button" | "input" | "image";

// Element class
// Virtual - does not represent the actual element
export default class Element {
  tagName: ElementTag;
  iid: string;
  _symbols: Map<string, string[]> = new Map();

  private _window: Window;

  constructor(tag: ElementTag, iid: string, _window: Window) {
    // Save parameters as properties
    this.tagName = tag;
    this.iid = iid;
    this._window = _window;
  }

  // Style object
  // HTML DOM alike
  style: {
    [key: string]: any;
    setProperty: (key: string, value: string, modifier?: string) => void;
  } = {
    // Function to set a css property
    // HTML DOM alike
    setProperty: (key: string, value: string, modifier: string = "_") => {
      // If the modifier is "_" (none), set the property directly
      if (modifier == "_") this.style[key] = value;
      else {
        // If the modifier object does not exist, create it
        if (!this.style[modifier]) this.style[modifier] = {};

        // Set the property in the modifier object
        this.style[modifier][key] = value;
      }

      // Transform the styles object into a css string
      const transformed = cssTransformer(this.iid, this.style);

      // Set the styles in the core (FFI set_styles function)
      this._window.core.set_styles(
        this._window.getChannelPtr() as Pointer,
        toCString(this.iid),
        toCString(transformed)
      );
    },
  };

  // Function to append a child to the element
  appendChild(child: Element) {
    // Append the child in the core (FFI append_child function)
    this._window.core.append_child(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid),
      toCString(child.iid)
    );
  }

  // Function to insert a child before another child
  insertBefore(child: Element, before: Element) {
    // Insert the child in the core (FFI insert_before function)
    this._window.core.insert_before(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid),
      toCString(child.iid),
      toCString(before.iid)
    );
  }

  // Function to set an attribute
  setAttribute(key: string, value: string) {
    // Set the attribute in the core (FFI set_attribute function)
    this._window.core.set_attribute(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid),
      toCString(this.tagName),
      toCString(key),
      toCString(value)
    );
  }

  // Function to get an attribute
  getAttribute(key: string) {
    // Generate a random id for the symbol
    const sid = randomId();

    // Get the shared buffer as Int32Array
    // Used for Atomics.wait blocking
    const i32 = new Int32Array(this._window.shared);

    // Request the core to get the attribute (FFI get_attribute function)
    this._window.core.get_attribute(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid),
      toCString(this.tagName),
      toCString(key),
      toCString(sid)
    );

    // Wait for the response
    const _status = Atomics.wait(i32, 0, 0, 10);

    // Read the result
    let message = "";

    // While loop to prevent race conditions
    while (true) {
      const obj = receiveMessageOnPort(this._window.localPort);

      if (!obj) break;

      message = obj.message;
    }

    return message;
  }

  // Function to add an event listener
  addEventListener(event: string, listener: () => void) {
    // Generate a random id for the symbol
    const symbol_id = randomId();

    // Save the listener in the element listeners object
    this._window.element_listeners[symbol_id] = listener;

    // Save the symbol id in the symbols map for the specific event
    this._symbols.set(event, [...(this._symbols.get(event) || []), symbol_id]);

    // Add the event listener in the core (FFI add_event_listener function)
    this._window.core.add_event_listener(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid),
      toCString(event),
      toCString(symbol_id)
    );

    return symbol_id;
  }

  // Function to remove an event listener
  // TODO: Implement this in core to improve performance
  removeEventListener(event: string) {
    // Get the symbol ids for the event
    const symbol_ids = this._symbols.get(event);

    // If there are no symbol ids, return
    if (!symbol_ids) return;

    // For each symbol id, remove the listener from the element listeners object
    symbol_ids.forEach((symbol_id) => {
      this._window.element_listeners[symbol_id] = () => {};
      delete this._window.element_listeners[symbol_id];
    });
  }

  // Function to remove the element
  remove() {
    // Remove the element in the core (FFI remove_element function)
    this._window.core.remove_element(
      this._window.getChannelPtr() as Pointer,
      toCString(this.iid)
    );

    // Delete the element virtually
    this._window.document.deleteElement(this.iid);
  }
}
