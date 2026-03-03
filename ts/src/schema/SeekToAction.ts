import { TypedSchema } from "../TypedSchema.js";

export interface SeekToActionOptions {
	target: string;
	startOffsetInput: string;
}

export class SeekToAction extends TypedSchema {
	static readonly schemaType = "SeekToAction";
	static readonly propertyMap: Record<string, string> = {
		startOffsetInput: "startOffset-input",
	};

	public readonly target: string;
	public readonly startOffsetInput: string;

	constructor(options: SeekToActionOptions) {
		super();
		this.target = options.target;
		this.startOffsetInput = options.startOffsetInput;
	}
}
