import { TypedSchema } from "../TypedSchema.js";

export interface ListItemOptions {
	position: number;
	name?: string | null;
	item?: string | TypedSchema | null;
	url?: string | null;
}

export class ListItem extends TypedSchema {
	static readonly schemaType = "ListItem";

	public readonly position: number;
	public readonly name: string | null;
	public readonly item: string | TypedSchema | null;
	public readonly url: string | null;

	constructor(options: ListItemOptions) {
		super();
		this.position = options.position;
		this.name = options.name ?? null;
		this.item = options.item ?? null;
		this.url = options.url ?? null;
	}
}
