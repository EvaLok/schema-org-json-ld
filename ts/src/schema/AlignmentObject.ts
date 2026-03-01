import { TypedSchema } from "../TypedSchema.js";

export class AlignmentObject extends TypedSchema {
	static readonly schemaType = "AlignmentObject";

	constructor(
		public readonly alignmentType: string,
		public readonly targetName: string,
		public readonly educationalFramework: string | null = null,
		public readonly targetUrl: string | null = null,
	) {
		super();
	}
}
