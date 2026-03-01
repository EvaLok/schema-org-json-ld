import { TypedSchema } from "../TypedSchema.js";
import type { InteractionCounter } from "./InteractionCounter.js";
import type { Organization } from "./Organization.js";
import type { PostalAddress } from "./PostalAddress.js";

export interface PersonOptions {
	name: string;
	url?: string | null;
	image?: string | null;
	email?: string | null;
	telephone?: string | null;
	jobTitle?: string | null;
	worksFor?: Organization | null;
	sameAs?: readonly string[] | null;
	description?: string | null;
	givenName?: string | null;
	familyName?: string | null;
	address?: PostalAddress | null;
	interactionStatistic?:
		| InteractionCounter
		| readonly InteractionCounter[]
		| null;
	agentInteractionStatistic?: InteractionCounter | null;
	identifier?: string | null;
	alternateName?: string | null;
}

export class Person extends TypedSchema {
	static readonly schemaType = "Person";

	public readonly name: string;
	public readonly url: string | null;
	public readonly image: string | null;
	public readonly email: string | null;
	public readonly telephone: string | null;
	public readonly jobTitle: string | null;
	public readonly worksFor: Organization | null;
	public readonly sameAs: readonly string[] | null;
	public readonly description: string | null;
	public readonly givenName: string | null;
	public readonly familyName: string | null;
	public readonly address: PostalAddress | null;
	public readonly interactionStatistic:
		| InteractionCounter
		| readonly InteractionCounter[]
		| null;
	public readonly agentInteractionStatistic: InteractionCounter | null;
	public readonly identifier: string | null;
	public readonly alternateName: string | null;

	constructor(options: PersonOptions) {
		super();
		this.name = options.name;
		this.url = options.url ?? null;
		this.image = options.image ?? null;
		this.email = options.email ?? null;
		this.telephone = options.telephone ?? null;
		this.jobTitle = options.jobTitle ?? null;
		this.worksFor = options.worksFor ?? null;
		this.sameAs = options.sameAs ?? null;
		this.description = options.description ?? null;
		this.givenName = options.givenName ?? null;
		this.familyName = options.familyName ?? null;
		this.address = options.address ?? null;
		this.interactionStatistic = options.interactionStatistic ?? null;
		this.agentInteractionStatistic = options.agentInteractionStatistic ?? null;
		this.identifier = options.identifier ?? null;
		this.alternateName = options.alternateName ?? null;
	}
}
