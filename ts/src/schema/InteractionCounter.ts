import { TypedSchema } from "../TypedSchema.js";

export class InteractionCounter extends TypedSchema {
	static readonly schemaType = "InteractionCounter";

	constructor(
		public readonly interactionType: string,
		public readonly userInteractionCount: number,
		public readonly interactionService: string | null = null,
	) {
		super();
	}
}
