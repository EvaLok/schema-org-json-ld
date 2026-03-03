import { TypedSchema } from "../TypedSchema.js";
import type { DayOfWeek } from "../enum/DayOfWeek.js";

export interface OpeningHoursSpecificationOptions {
	dayOfWeek?: DayOfWeek | null;
	opens?: string | null;
	closes?: string | null;
	validFrom?: string | null;
	validThrough?: string | null;
}

export class OpeningHoursSpecification extends TypedSchema {
	static readonly schemaType = "OpeningHoursSpecification";

	public readonly dayOfWeek: DayOfWeek | null;
	public readonly opens: string | null;
	public readonly closes: string | null;
	public readonly validFrom: string | null;
	public readonly validThrough: string | null;

	constructor(options: OpeningHoursSpecificationOptions = {}) {
		super();
		this.dayOfWeek = options.dayOfWeek ?? null;
		this.opens = options.opens ?? null;
		this.closes = options.closes ?? null;
		this.validFrom = options.validFrom ?? null;
		this.validThrough = options.validThrough ?? null;
	}
}
