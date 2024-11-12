import type Window from "./window";
export type ElementTag = "view" | "text" | "button" | "input";
export default class Element {
    tagName: ElementTag;
    iid: string;
    _symbols: Map<string, string[]>;
    private _window;
    constructor(tag: ElementTag, iid: string, _window: Window);
    style: {
        [key: string]: any;
        setProperty: (key: string, value: string, modifier?: string) => void;
    };
    appendChild(child: Element): void;
    insertBefore(child: Element, before: Element): void;
    setAttribute(key: string, value: string): void;
    getAttribute(key: string): string;
    addEventListener(event: string, listener: () => void): string;
    removeEventListener(event: string): void;
    remove(): void;
}
