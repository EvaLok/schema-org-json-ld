import { TypedSchema } from "../TypedSchema.js";
import type { HowToStep } from "./HowToStep.js";

export interface HowToSectionOptions {
	name: string;
	itemListElement: readonly HowToStep[];
}

export class HowToSection extends TypedSchema {
	static readonly schemaType = "HowToSection";

	public readonly name: string;
	public readonly itemListElement: readonly HowToStep[];

	constructor(options: HowToSectionOptions) {
		super();
		this.name = options.name;
		this.itemListElement = options.itemListElement;
	}
}
