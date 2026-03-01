import { TypedSchema } from "../TypedSchema.js";
import type { HowToStep } from "./HowToStep.js";

export class HowToSection extends TypedSchema {
	static readonly schemaType = "HowToSection";

	constructor(
		public readonly name: string,
		public readonly itemListElement: readonly HowToStep[],
	) {
		super();
	}
}
