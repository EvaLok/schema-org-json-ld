import { TypedSchema } from "../TypedSchema.js";

export class Schedule extends TypedSchema {
	static readonly schemaType = "Schedule";

	constructor(
		public readonly repeatFrequency: string,
		public readonly repeatCount: number | null = null,
		public readonly startDate: string | null = null,
		public readonly endDate: string | null = null,
	) {
		super();
	}
}
