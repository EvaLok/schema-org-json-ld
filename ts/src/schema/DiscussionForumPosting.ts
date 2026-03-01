import { TypedSchema } from "../TypedSchema.js";
import type { Comment } from "./Comment.js";
import type { ImageObject } from "./ImageObject.js";
import type { InteractionCounter } from "./InteractionCounter.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";
import type { VideoObject } from "./VideoObject.js";

export interface DiscussionForumPostingOptions {
	author: Person | Organization;
	datePublished: string;
	text: string;
	headline?: string | null;
	url?: string | null;
	dateModified?: string | null;
	image?: ImageObject | null;
	video?: VideoObject | null;
	comment?: readonly Comment[] | null;
	interactionStatistic?:
		| InteractionCounter
		| readonly InteractionCounter[]
		| null;
	isPartOf?: string | null;
	sharedContent?: string | null;
	creativeWorkStatus?: string | null;
	mainEntityOfPage?: string | null;
}

export class DiscussionForumPosting extends TypedSchema {
	static readonly schemaType = "DiscussionForumPosting";

	public readonly author: Person | Organization;
	public readonly datePublished: string;
	public readonly text: string;
	public readonly headline: string | null;
	public readonly url: string | null;
	public readonly dateModified: string | null;
	public readonly image: ImageObject | null;
	public readonly video: VideoObject | null;
	public readonly comment: readonly Comment[] | null;
	public readonly interactionStatistic:
		| InteractionCounter
		| readonly InteractionCounter[]
		| null;
	public readonly isPartOf: string | null;
	public readonly sharedContent: string | null;
	public readonly creativeWorkStatus: string | null;
	public readonly mainEntityOfPage: string | null;

	constructor(options: DiscussionForumPostingOptions) {
		super();
		this.author = options.author;
		this.datePublished = options.datePublished;
		this.text = options.text;
		this.headline = options.headline ?? null;
		this.url = options.url ?? null;
		this.dateModified = options.dateModified ?? null;
		this.image = options.image ?? null;
		this.video = options.video ?? null;
		this.comment = options.comment ?? null;
		this.interactionStatistic = options.interactionStatistic ?? null;
		this.isPartOf = options.isPartOf ?? null;
		this.sharedContent = options.sharedContent ?? null;
		this.creativeWorkStatus = options.creativeWorkStatus ?? null;
		this.mainEntityOfPage = options.mainEntityOfPage ?? null;
	}
}
