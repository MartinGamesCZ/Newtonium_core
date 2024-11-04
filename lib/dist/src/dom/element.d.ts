import type Window from "./window";
export type ElementTag = "view" | "text" | "button";
export default class Element {
    tagName: ElementTag;
    iid: string;
    private _window;
    constructor(tag: ElementTag, iid: string, _window: Window);
    style: {
        [key: string]: any;
        setProperty: (key: string, value: string) => void;
    };
    appendChild(child: Element): void;
    setAttribute(key: string, value: string): void;
    addEventListener(event: string, listener: () => void): void;
    remove(): void;
}
