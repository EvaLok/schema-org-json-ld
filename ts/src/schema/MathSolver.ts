import { TypedSchema } from "../TypedSchema.js";
import type { SolveMathAction } from "./SolveMathAction.js";

export class MathSolver extends TypedSchema {
	static readonly schemaType = ["MathSolver", "LearningResource"];

	constructor(
		public readonly url: string,
		public readonly usageInfo: string,
		public readonly potentialAction:
			| SolveMathAction
			| readonly SolveMathAction[],
		public readonly name: string | null = null,
		public readonly inLanguage: string | null = null,
		public readonly learningResourceType: string | null = null,
		public readonly assesses: readonly string[] | null = null,
	) {
		super();
	}
}
