import { TypedSchema } from "../TypedSchema.js";
import type { Clip } from "./Clip.js";

export class HowToStep extends TypedSchema {
	static readonly schemaType = "HowToStep";

	constructor(
		public readonly text: string,
		public readonly name: string | null = null,
		public readonly url: string | null = null,
		public readonly image: string | null = null,
		public readonly video: Clip | null = null,
		public readonly itemListElement: readonly string[] | null = null,
	) {
		super();
	}
}
