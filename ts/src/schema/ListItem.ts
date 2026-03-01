import { TypedSchema } from "../TypedSchema.js";

export class ListItem extends TypedSchema {
	static readonly schemaType = "ListItem";

	constructor(
		public readonly position: number,
		public readonly name: string | null = null,
		public readonly item: string | TypedSchema | null = null,
		public readonly url: string | null = null,
	) {
		super();
	}
}
