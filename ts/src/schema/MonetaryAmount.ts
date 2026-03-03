import { TypedSchema } from "../TypedSchema.js";

export interface MonetaryAmountOptions {
	currency: string;
	value?: number | null;
	minValue?: number | null;
	maxValue?: number | null;
	unitText?: string | null;
}

export class MonetaryAmount extends TypedSchema {
	static readonly schemaType = "MonetaryAmount";

	public readonly currency: string;
	public readonly value: number | null;
	public readonly minValue: number | null;
	public readonly maxValue: number | null;
	public readonly unitText: string | null;

	constructor(options: MonetaryAmountOptions) {
		super();
		this.currency = options.currency;
		this.value = options.value ?? null;
		this.minValue = options.minValue ?? null;
		this.maxValue = options.maxValue ?? null;
		this.unitText = options.unitText ?? null;
	}
}
