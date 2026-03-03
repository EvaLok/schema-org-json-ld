import { TypedSchema } from "../TypedSchema.js";
import type { DayOfWeek } from "../enum/DayOfWeek.js";
import type { QuantitativeValue } from "./QuantitativeValue.js";

export interface ServicePeriodOptions {
	duration?: QuantitativeValue | null;
	businessDays?: readonly DayOfWeek[] | null;
	cutoffTime?: string | null;
}

export class ServicePeriod extends TypedSchema {
	static readonly schemaType = "ServicePeriod";

	public readonly duration: QuantitativeValue | null;
	public readonly businessDays: readonly DayOfWeek[] | null;
	public readonly cutoffTime: string | null;

	constructor(options: ServicePeriodOptions = {}) {
		super();
		this.duration = options.duration ?? null;
		this.businessDays = options.businessDays ?? null;
		this.cutoffTime = options.cutoffTime ?? null;
	}
}
