import { TypedSchema } from "../TypedSchema.js";
import type { BroadcastEvent } from "./BroadcastEvent.js";
import type { Clip } from "./Clip.js";
import type { InteractionCounter } from "./InteractionCounter.js";

export interface VideoObjectOptions {
	name: string;
	thumbnailUrl: readonly string[];
	uploadDate: string;
	description?: string | null;
	contentUrl?: string | null;
	embedUrl?: string | null;
	duration?: string | null;
	expires?: string | null;
	regionsAllowed?: string | null;
	interactionStatistic?:
		| InteractionCounter
		| readonly InteractionCounter[]
		| null;
	hasPart?: readonly Clip[] | null;
	ineligibleRegion?: string | null;
	publication?: BroadcastEvent | null;
}

export class VideoObject extends TypedSchema {
	static readonly schemaType = "VideoObject";

	public readonly name: string;
	public readonly thumbnailUrl: readonly string[];
	public readonly uploadDate: string;
	public readonly description: string | null;
	public readonly contentUrl: string | null;
	public readonly embedUrl: string | null;
	public readonly duration: string | null;
	public readonly expires: string | null;
	public readonly regionsAllowed: string | null;
	public readonly interactionStatistic:
		| InteractionCounter
		| readonly InteractionCounter[]
		| null;
	public readonly hasPart: readonly Clip[] | null;
	public readonly ineligibleRegion: string | null;
	public readonly publication: BroadcastEvent | null;

	constructor(options: VideoObjectOptions) {
		super();
		this.name = options.name;
		this.thumbnailUrl = options.thumbnailUrl;
		this.uploadDate = options.uploadDate;
		this.description = options.description ?? null;
		this.contentUrl = options.contentUrl ?? null;
		this.embedUrl = options.embedUrl ?? null;
		this.duration = options.duration ?? null;
		this.expires = options.expires ?? null;
		this.regionsAllowed = options.regionsAllowed ?? null;
		this.interactionStatistic = options.interactionStatistic ?? null;
		this.hasPart = options.hasPart ?? null;
		this.ineligibleRegion = options.ineligibleRegion ?? null;
		this.publication = options.publication ?? null;
	}
}
