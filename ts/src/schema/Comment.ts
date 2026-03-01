import { TypedSchema } from "../TypedSchema.js";
import type { ImageObject } from "./ImageObject.js";
import type { InteractionCounter } from "./InteractionCounter.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";
import type { VideoObject } from "./VideoObject.js";

export interface CommentOptions {
	text: string;
	author?: Person | Organization | null;
	datePublished?: string | null;
	url?: string | null;
	dateModified?: string | null;
	image?: ImageObject | null;
	video?: VideoObject | null;
	comment?: readonly Comment[] | null;
	interactionStatistic?:
		| InteractionCounter
		| readonly InteractionCounter[]
		| null;
	sharedContent?: string | null;
	creativeWorkStatus?: string | null;
}

export class Comment extends TypedSchema {
	static readonly schemaType = "Comment";

	public readonly text: string;
	public readonly author: Person | Organization | null;
	public readonly datePublished: string | null;
	public readonly url: string | null;
	public readonly dateModified: string | null;
	public readonly image: ImageObject | null;
	public readonly video: VideoObject | null;
	public readonly comment: readonly Comment[] | null;
	public readonly interactionStatistic:
		| InteractionCounter
		| readonly InteractionCounter[]
		| null;
	public readonly sharedContent: string | null;
	public readonly creativeWorkStatus: string | null;

	constructor(options: CommentOptions) {
		super();
		this.text = options.text;
		this.author = options.author ?? null;
		this.datePublished = options.datePublished ?? null;
		this.url = options.url ?? null;
		this.dateModified = options.dateModified ?? null;
		this.image = options.image ?? null;
		this.video = options.video ?? null;
		this.comment = options.comment ?? null;
		this.interactionStatistic = options.interactionStatistic ?? null;
		this.sharedContent = options.sharedContent ?? null;
		this.creativeWorkStatus = options.creativeWorkStatus ?? null;
	}
}
