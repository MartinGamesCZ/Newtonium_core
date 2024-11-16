import { randomUUID } from "crypto";
import Element, { type ElementTag } from "./element";
import { randomId } from "../utils/id";
import util from "util";
import type Window from "./window";
import { toCString } from "../ffi";
import type { Pointer } from "bun:ffi";

// Document class
// Made to represent the document of a window
export default class Document {
  private _elements: Map<
    string,
    {
      pointer: number;
      element: Element;
    }
  > = new Map();
  private _window: Window;

  body: Element;

  constructor(window: Window) {
    // Save parameters as properties
    this._window = window;

    // Create a new body element
    // Only saves the element, doesn't create it (it's created by Rust when creating a window)
    this.body = this._createElementWithId("view", "body", true);
  }

  // Function to create a new element
  // HTML DOM alike
  createElement(tag: ElementTag, args: { [key: string]: any } = {}) {
    // Generate a random id
    const iid = randomId();

    // Create the element (using the internal function)
    return this._createElementWithId(tag, iid, false, args);
  }

  // Function to get an element by id
  getElementById(id: string) {
    // Return the element by the given id
    return this._elements.get(id)?.element;
  }

  // Internal function to create the element
  // Not HTML DOM alike, allows to skip the creation
  private _createElementWithId(
    tag: ElementTag,
    iid: string,
    skipCreate = false,
    args: {
      [key: string]: any;
    } = {}
  ) {
    // Create the element in core if not skiped
    if (!skipCreate)
      this._window.core.create_element(
        this._window.getChannelPtr() as Pointer,
        toCString(tag),
        toCString(iid),
        toCString(JSON.stringify(args))
      );

    // Create a new element class instance
    // Virtual, doesn't create the element in core
    const element = new Element(tag, iid, this._window);

    // Save the element in the elements map
    this._elements.set(iid, {
      pointer: 0,
      element,
    });

    return element;
  }

  // Function to delete an element (virtually)
  deleteElement(iid: string) {
    // Remove the element from the elements map
    this._elements.delete(iid);
  }

  // Custom inspect function
  [util.inspect.custom](_depth: number, _options: any) {
    return `NewtoniumDocument {
  elements: ${this._elements.size}
}`;
  }
}
