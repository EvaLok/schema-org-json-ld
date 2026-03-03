import { TypedSchema } from "../TypedSchema.js";

export interface AlignmentObjectOptions {
	alignmentType: string;
	targetName: string;
	educationalFramework?: string | null;
	targetUrl?: string | null;
}

export class AlignmentObject extends TypedSchema {
	static readonly schemaType = "AlignmentObject";

	public readonly alignmentType: string;
	public readonly targetName: string;
	public readonly educationalFramework: string | null;
	public readonly targetUrl: string | null;

	constructor(options: AlignmentObjectOptions) {
		super();
		this.alignmentType = options.alignmentType;
		this.targetName = options.targetName;
		this.educationalFramework = options.educationalFramework ?? null;
		this.targetUrl = options.targetUrl ?? null;
	}
}
