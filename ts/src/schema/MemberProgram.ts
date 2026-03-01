import { TypedSchema } from "../TypedSchema.js";
import type { MemberProgramTier } from "./MemberProgramTier.js";

export class MemberProgram extends TypedSchema {
	static readonly schemaType = "MemberProgram";

	constructor(
		public readonly name: string,
		public readonly description: string,
		public readonly hasTiers: readonly MemberProgramTier[],
		public readonly url: string | null = null,
	) {
		super();
	}
}
