import { TypedSchema } from "../TypedSchema.js";
import type { SolveMathAction } from "./SolveMathAction.js";

export interface MathSolverOptions {
	url: string;
	usageInfo: string;
	potentialAction: SolveMathAction | readonly SolveMathAction[];
	name?: string | null;
	inLanguage?: string | null;
	learningResourceType?: string | null;
	assesses?: readonly string[] | null;
}

export class MathSolver extends TypedSchema {
	static readonly schemaType = ["MathSolver", "LearningResource"];

	public readonly url: string;
	public readonly usageInfo: string;
	public readonly potentialAction: SolveMathAction | readonly SolveMathAction[];
	public readonly name: string | null;
	public readonly inLanguage: string | null;
	public readonly learningResourceType: string | null;
	public readonly assesses: readonly string[] | null;

	constructor(options: MathSolverOptions) {
		super();
		this.url = options.url;
		this.usageInfo = options.usageInfo;
		this.potentialAction = options.potentialAction;
		this.name = options.name ?? null;
		this.inLanguage = options.inLanguage ?? null;
		this.learningResourceType = options.learningResourceType ?? null;
		this.assesses = options.assesses ?? null;
	}
}
