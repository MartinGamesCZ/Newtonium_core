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
        setProperty: (key: string, value: string) => void;
    };
    appendChild(child: Element): void;
    setAttribute(key: string, value: string): void;
    getAttribute(key: string): Promise<unknown>;
    addEventListener(event: string, listener: () => void): string;
    removeEventListener(event: string): void;
    remove(): void;
}
