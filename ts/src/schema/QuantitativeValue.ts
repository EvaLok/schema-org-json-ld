import { TypedSchema } from "../TypedSchema.js";

export interface QuantitativeValueOptions {
	value?: number | null;
	unitCode?: string | null;
	minValue?: number | null;
	maxValue?: number | null;
}

export class QuantitativeValue extends TypedSchema {
	static readonly schemaType = "QuantitativeValue";

	public readonly value: number | null;
	public readonly unitCode: string | null;
	public readonly minValue: number | null;
	public readonly maxValue: number | null;

	constructor(options: QuantitativeValueOptions = {}) {
		super();
		this.value = options.value ?? null;
		this.unitCode = options.unitCode ?? null;
		this.minValue = options.minValue ?? null;
		this.maxValue = options.maxValue ?? null;
	}
}
