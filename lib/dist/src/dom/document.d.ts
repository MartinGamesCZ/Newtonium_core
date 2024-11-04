import Element, { type ElementTag } from "./element";
import util from "util";
import type Window from "./window";
export default class Document {
    private _elements;
    private _window;
    body: Element;
    constructor(window: Window);
    createElement(tag: ElementTag, args?: {
        [key: string]: any;
    }): Element;
    getElementById(id: string): Element | undefined;
    private _createElementWithId;
    deleteElement(iid: string): void;
    [util.inspect.custom](_depth: number, _options: any): string;
}
