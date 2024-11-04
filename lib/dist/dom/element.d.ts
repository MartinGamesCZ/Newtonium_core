export type ElementTag = "view" | "text" | "button";
export default class Element {
    tagName: ElementTag;
    iid: string;
    constructor(tag: ElementTag, iid: string);
}
