import { TypedSchema } from "../TypedSchema.js";
import type { ListItem } from "./ListItem.js";

export class BreadcrumbList extends TypedSchema {
static readonly schemaType = "BreadcrumbList";

constructor(public readonly itemListElement: readonly ListItem[]) {
super();
}
}
