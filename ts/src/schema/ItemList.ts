import { TypedSchema } from "../TypedSchema.js";
import type { ListItem } from "./ListItem.js";

export class ItemList extends TypedSchema {
	static readonly schemaType = "ItemList";

	constructor(
		public readonly itemListElement: readonly ListItem[],
		public readonly itemListOrder: string | null = null,
		public readonly numberOfItems: number | null = null,
	) {
		super();
	}
}
