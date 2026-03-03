import { TypedSchema } from "../TypedSchema.js";

export interface BroadcastEventOptions {
	isLiveBroadcast: boolean;
	startDate?: string | null;
	endDate?: string | null;
}

export class BroadcastEvent extends TypedSchema {
	static readonly schemaType = "BroadcastEvent";

	public readonly isLiveBroadcast: boolean;
	public readonly startDate: string | null;
	public readonly endDate: string | null;

	constructor(options: BroadcastEventOptions) {
		super();
		this.isLiveBroadcast = options.isLiveBroadcast;
		this.startDate = options.startDate ?? null;
		this.endDate = options.endDate ?? null;
	}
}
