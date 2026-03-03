import { TypedSchema } from "../TypedSchema.js";
import type { MemberProgramTier } from "./MemberProgramTier.js";

export interface MemberProgramOptions {
	name: string;
	description: string;
	hasTiers: readonly MemberProgramTier[];
	url?: string | null;
}

export class MemberProgram extends TypedSchema {
	static readonly schemaType = "MemberProgram";

	public readonly name: string;
	public readonly description: string;
	public readonly hasTiers: readonly MemberProgramTier[];
	public readonly url: string | null;

	constructor(options: MemberProgramOptions) {
		super();
		this.name = options.name;
		this.description = options.description;
		this.hasTiers = options.hasTiers;
		this.url = options.url ?? null;
	}
}
