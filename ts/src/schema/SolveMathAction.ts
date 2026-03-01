import { TypedSchema } from "../TypedSchema.js";

export class SolveMathAction extends TypedSchema {
	static readonly schemaType = "SolveMathAction";
	static readonly propertyMap: Record<string, string> = {
		mathExpressionInput: "mathExpression-input",
	};

	constructor(
		public readonly target: string,
		public readonly mathExpressionInput: string,
		public readonly eduQuestionType: string | readonly string[] | null = null,
	) {
		super();
	}
}
