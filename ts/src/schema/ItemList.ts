import { TypedSchema } from "../TypedSchema.js";
import type { ListItem } from "./ListItem.js";

export interface ItemListOptions {
	itemListElement: readonly ListItem[];
	itemListOrder?: string | null;
	numberOfItems?: number | null;
}

export class ItemList extends TypedSchema {
	static readonly schemaType = "ItemList";

	public readonly itemListElement: readonly ListItem[];
	public readonly itemListOrder: string | null;
	public readonly numberOfItems: number | null;

	constructor(options: ItemListOptions) {
		super();
		this.itemListElement = options.itemListElement;
		this.itemListOrder = options.itemListOrder ?? null;
		this.numberOfItems = options.numberOfItems ?? null;
	}
}
