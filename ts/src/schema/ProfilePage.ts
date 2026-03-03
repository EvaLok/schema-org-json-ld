import { TypedSchema } from "../TypedSchema.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";

export interface ProfilePageOptions {
	mainEntity: Person | Organization;
	dateCreated?: string | null;
	dateModified?: string | null;
}

export class ProfilePage extends TypedSchema {
	static readonly schemaType = "ProfilePage";

	public readonly mainEntity: Person | Organization;
	public readonly dateCreated: string | null;
	public readonly dateModified: string | null;

	constructor(options: ProfilePageOptions) {
		super();
		this.mainEntity = options.mainEntity;
		this.dateCreated = options.dateCreated ?? null;
		this.dateModified = options.dateModified ?? null;
	}
}
