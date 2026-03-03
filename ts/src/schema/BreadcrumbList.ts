import { TypedSchema } from "../TypedSchema.js";
import type { ListItem } from "./ListItem.js";

export interface BreadcrumbListOptions {
	itemListElement: readonly ListItem[];
}

export class BreadcrumbList extends TypedSchema {
	static readonly schemaType = "BreadcrumbList";

	public readonly itemListElement: readonly ListItem[];

	constructor(options: BreadcrumbListOptions) {
		super();
		this.itemListElement = options.itemListElement;
	}
}
