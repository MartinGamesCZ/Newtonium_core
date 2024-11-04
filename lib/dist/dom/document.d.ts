import Element, { type ElementTag } from "./element";
import util from "util";
export default class Document {
    private _elements;
    constructor();
    createElement(tag: ElementTag): Element;
    [util.inspect.custom](_depth: number, _options: any): string;
}
