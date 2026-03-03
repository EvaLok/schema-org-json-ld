import { TypedSchema } from "../TypedSchema.js";

export interface SolveMathActionOptions {
	target: string;
	mathExpressionInput: string;
	eduQuestionType?: string | readonly string[] | null;
}

export class SolveMathAction extends TypedSchema {
	static readonly schemaType = "SolveMathAction";
	static readonly propertyMap: Record<string, string> = {
		mathExpressionInput: "mathExpression-input",
	};

	public readonly target: string;
	public readonly mathExpressionInput: string;
	public readonly eduQuestionType: string | readonly string[] | null;

	constructor(options: SolveMathActionOptions) {
		super();
		this.target = options.target;
		this.mathExpressionInput = options.mathExpressionInput;
		this.eduQuestionType = options.eduQuestionType ?? null;
	}
}
