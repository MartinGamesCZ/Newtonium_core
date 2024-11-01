import { randomUUID } from "crypto";
import Element, { type ElementTag } from "./element";
import { randomId } from "../utils/id";
import util from "util";
import type Window from "./window";
import { toCString } from "../ffi";
import type { Pointer } from "bun:ffi";

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
    this._window = window;

    this.body = this._createElementWithId("view", "body", true);
  }

  createElement(tag: ElementTag) {
    const iid = randomId();

    return this._createElementWithId(tag, iid);
  }

  private _createElementWithId(
    tag: ElementTag,
    iid: string,
    skipCreate = false
  ) {
    if (!skipCreate)
      this._window.core.create_element(
        this._window.getChannelPtr() as Pointer,
        toCString(tag),
        toCString(iid),
        toCString("")
      );

    const element = new Element(tag, iid, this._window);

    this._elements.set(iid, {
      pointer: 0,
      element,
    });

    return element;
  }

  [util.inspect.custom](_depth: number, _options: any) {
    return `NewtoniumDocument {
  elements: ${this._elements.size}
}`;
  }
}
