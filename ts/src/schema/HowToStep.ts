import { TypedSchema } from "../TypedSchema.js";
import type { Clip } from "./Clip.js";

export interface HowToStepOptions {
	text: string;
	name?: string | null;
	url?: string | null;
	image?: string | null;
	video?: Clip | null;
	itemListElement?: readonly string[] | null;
}

export class HowToStep extends TypedSchema {
	static readonly schemaType = "HowToStep";

	public readonly text: string;
	public readonly name: string | null;
	public readonly url: string | null;
	public readonly image: string | null;
	public readonly video: Clip | null;
	public readonly itemListElement: readonly string[] | null;

	constructor(options: HowToStepOptions) {
		super();
		this.text = options.text;
		this.name = options.name ?? null;
		this.url = options.url ?? null;
		this.image = options.image ?? null;
		this.video = options.video ?? null;
		this.itemListElement = options.itemListElement ?? null;
	}
}
