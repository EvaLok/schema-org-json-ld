import { TypedSchema } from "../TypedSchema.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";

export class ProfilePage extends TypedSchema {
	static readonly schemaType = "ProfilePage";

	constructor(
		public readonly mainEntity: Person | Organization,
		public readonly dateCreated: string | null = null,
		public readonly dateModified: string | null = null,
	) {
		super();
	}
}
