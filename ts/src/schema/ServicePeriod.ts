import { TypedSchema } from "../TypedSchema.js";
import type { DayOfWeek } from "../enum/DayOfWeek.js";
import type { QuantitativeValue } from "./QuantitativeValue.js";

export class ServicePeriod extends TypedSchema {
	static readonly schemaType = "ServicePeriod";

	constructor(
		public readonly duration: QuantitativeValue | null = null,
		public readonly businessDays: readonly DayOfWeek[] | null = null,
		public readonly cutoffTime: string | null = null,
	) {
		super();
	}
}
