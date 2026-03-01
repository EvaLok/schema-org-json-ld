import { TypedSchema } from "../TypedSchema.js";
import type { ImageObject } from "./ImageObject.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";
import type { SpeakableSpecification } from "./SpeakableSpecification.js";
import type { WebPageElement } from "./WebPageElement.js";

export interface ArticleOptions {
	headline: string;
	author?: Person | Organization | readonly (Person | Organization)[] | null;
	datePublished?: string | null;
	dateModified?: string | null;
	image?: readonly (string | ImageObject)[] | null;
	description?: string | null;
	publisher?: Organization | null;
	speakable?: SpeakableSpecification | null;
	isAccessibleForFree?: boolean | null;
	hasPart?: WebPageElement | readonly WebPageElement[] | null;
}

export class Article extends TypedSchema {
	static readonly schemaType = "Article";

	public readonly headline: string;
	public readonly author:
		| Person
		| Organization
		| readonly (Person | Organization)[]
		| null;
	public readonly datePublished: string | null;
	public readonly dateModified: string | null;
	public readonly image: readonly (string | ImageObject)[] | null;
	public readonly description: string | null;
	public readonly publisher: Organization | null;
	public readonly speakable: SpeakableSpecification | null;
	public readonly isAccessibleForFree: boolean | null;
	public readonly hasPart: WebPageElement | readonly WebPageElement[] | null;

	constructor(options: ArticleOptions) {
		super();
		this.headline = options.headline;
		this.author = options.author ?? null;
		this.datePublished = options.datePublished ?? null;
		this.dateModified = options.dateModified ?? null;
		this.image = options.image ?? null;
		this.description = options.description ?? null;
		this.publisher = options.publisher ?? null;
		this.speakable = options.speakable ?? null;
		this.isAccessibleForFree = options.isAccessibleForFree ?? null;
		this.hasPart = options.hasPart ?? null;
	}
}
