import { TypedSchema } from "../TypedSchema.js";

export class BroadcastEvent extends TypedSchema {
	static readonly schemaType = "BroadcastEvent";

	constructor(
		public readonly isLiveBroadcast: boolean,
		public readonly startDate: string | null = null,
		public readonly endDate: string | null = null,
	) {
		super();
	}
}
