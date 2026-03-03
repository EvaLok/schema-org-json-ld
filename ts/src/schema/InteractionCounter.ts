import { TypedSchema } from "../TypedSchema.js";

export interface InteractionCounterOptions {
	interactionType: string;
	userInteractionCount: number;
	interactionService?: string | null;
}

export class InteractionCounter extends TypedSchema {
	static readonly schemaType = "InteractionCounter";

	public readonly interactionType: string;
	public readonly userInteractionCount: number;
	public readonly interactionService: string | null;

	constructor(options: InteractionCounterOptions) {
		super();
		this.interactionType = options.interactionType;
		this.userInteractionCount = options.userInteractionCount;
		this.interactionService = options.interactionService ?? null;
	}
}
