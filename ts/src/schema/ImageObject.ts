import { TypedSchema } from "../TypedSchema.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";

export interface ImageObjectOptions {
	contentUrl: string;
	url?: string | null;
	name?: string | null;
	caption?: string | null;
	description?: string | null;
	width?: string | null;
	height?: string | null;
	license?: string | null;
	acquireLicensePage?: string | null;
	creditText?: string | null;
	copyrightNotice?: string | null;
	creator?: Organization | Person | null;
	datePublished?: string | null;
	uploadDate?: string | null;
}

export class ImageObject extends TypedSchema {
	static readonly schemaType = "ImageObject";

	public readonly contentUrl: string;
	public readonly url: string | null;
	public readonly name: string | null;
	public readonly caption: string | null;
	public readonly description: string | null;
	public readonly width: string | null;
	public readonly height: string | null;
	public readonly license: string | null;
	public readonly acquireLicensePage: string | null;
	public readonly creditText: string | null;
	public readonly copyrightNotice: string | null;
	public readonly creator: Organization | Person | null;
	public readonly datePublished: string | null;
	public readonly uploadDate: string | null;

	constructor(options: ImageObjectOptions) {
		super();
		this.contentUrl = options.contentUrl;
		this.url = options.url ?? null;
		this.name = options.name ?? null;
		this.caption = options.caption ?? null;
		this.description = options.description ?? null;
		this.width = options.width ?? null;
		this.height = options.height ?? null;
		this.license = options.license ?? null;
		this.acquireLicensePage = options.acquireLicensePage ?? null;
		this.creditText = options.creditText ?? null;
		this.copyrightNotice = options.copyrightNotice ?? null;
		this.creator = options.creator ?? null;
		this.datePublished = options.datePublished ?? null;
		this.uploadDate = options.uploadDate ?? null;
	}
}
