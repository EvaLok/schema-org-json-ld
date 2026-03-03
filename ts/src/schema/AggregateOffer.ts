import { TypedSchema } from "../TypedSchema.js";

export interface AggregateOfferOptions {
	lowPrice: number;
	priceCurrency: string;
	highPrice?: number | null;
	offerCount?: number | null;
}

export class AggregateOffer extends TypedSchema {
	static readonly schemaType = "AggregateOffer";

	public readonly lowPrice: number;
	public readonly priceCurrency: string;
	public readonly highPrice: number | null;
	public readonly offerCount: number | null;

	constructor(options: AggregateOfferOptions) {
		super();
		this.lowPrice = options.lowPrice;
		this.priceCurrency = options.priceCurrency;
		this.highPrice = options.highPrice ?? null;
		this.offerCount = options.offerCount ?? null;
	}
}
