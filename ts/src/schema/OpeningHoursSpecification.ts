import { TypedSchema } from "../TypedSchema.js";
import type { DayOfWeek } from "../enum/DayOfWeek.js";

export class OpeningHoursSpecification extends TypedSchema {
	static readonly schemaType = "OpeningHoursSpecification";

	constructor(
		public readonly dayOfWeek: DayOfWeek | null = null,
		public readonly opens: string | null = null,
		public readonly closes: string | null = null,
		public readonly validFrom: string | null = null,
		public readonly validThrough: string | null = null,
	) {
		super();
	}
}
