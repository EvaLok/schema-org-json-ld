import { TypedSchema } from "../TypedSchema.js";
import type { ContactPoint } from "./ContactPoint.js";
import type { MemberProgram } from "./MemberProgram.js";
import type { MerchantReturnPolicy } from "./MerchantReturnPolicy.js";
import type { PostalAddress } from "./PostalAddress.js";
import type { QuantitativeValue } from "./QuantitativeValue.js";
import type { ShippingService } from "./ShippingService.js";

export interface OrganizationOptions {
	name: string;
	url?: string | null;
	logo?: string | null;
	description?: string | null;
	email?: string | null;
	telephone?: string | null;
	address?: PostalAddress | null;
	contactPoint?: ContactPoint | null;
	sameAs?: readonly string[] | null;
	foundingDate?: string | null;
	alternateName?: string | null;
	legalName?: string | null;
	numberOfEmployees?: QuantitativeValue | null;
	taxID?: string | null;
	vatID?: string | null;
	naics?: string | null;
	duns?: string | null;
	leiCode?: string | null;
	iso6523Code?: string | null;
	globalLocationNumber?: string | null;
	hasMerchantReturnPolicy?:
		| MerchantReturnPolicy
		| readonly MerchantReturnPolicy[]
		| null;
	hasMemberProgram?: MemberProgram | readonly MemberProgram[] | null;
	hasShippingService?: ShippingService | readonly ShippingService[] | null;
}

export class Organization extends TypedSchema {
	static readonly schemaType = "Organization";

	public readonly name: string;
	public readonly url: string | null;
	public readonly logo: string | null;
	public readonly description: string | null;
	public readonly email: string | null;
	public readonly telephone: string | null;
	public readonly address: PostalAddress | null;
	public readonly contactPoint: ContactPoint | null;
	public readonly sameAs: readonly string[] | null;
	public readonly foundingDate: string | null;
	public readonly alternateName: string | null;
	public readonly legalName: string | null;
	public readonly numberOfEmployees: QuantitativeValue | null;
	public readonly taxID: string | null;
	public readonly vatID: string | null;
	public readonly naics: string | null;
	public readonly duns: string | null;
	public readonly leiCode: string | null;
	public readonly iso6523Code: string | null;
	public readonly globalLocationNumber: string | null;
	public readonly hasMerchantReturnPolicy:
		| MerchantReturnPolicy
		| readonly MerchantReturnPolicy[]
		| null;
	public readonly hasMemberProgram:
		| MemberProgram
		| readonly MemberProgram[]
		| null;
	public readonly hasShippingService:
		| ShippingService
		| readonly ShippingService[]
		| null;

	constructor(options: OrganizationOptions) {
		super();
		this.name = options.name;
		this.url = options.url ?? null;
		this.logo = options.logo ?? null;
		this.description = options.description ?? null;
		this.email = options.email ?? null;
		this.telephone = options.telephone ?? null;
		this.address = options.address ?? null;
		this.contactPoint = options.contactPoint ?? null;
		this.sameAs = options.sameAs ?? null;
		this.foundingDate = options.foundingDate ?? null;
		this.alternateName = options.alternateName ?? null;
		this.legalName = options.legalName ?? null;
		this.numberOfEmployees = options.numberOfEmployees ?? null;
		this.taxID = options.taxID ?? null;
		this.vatID = options.vatID ?? null;
		this.naics = options.naics ?? null;
		this.duns = options.duns ?? null;
		this.leiCode = options.leiCode ?? null;
		this.iso6523Code = options.iso6523Code ?? null;
		this.globalLocationNumber = options.globalLocationNumber ?? null;
		this.hasMerchantReturnPolicy = options.hasMerchantReturnPolicy ?? null;
		this.hasMemberProgram = options.hasMemberProgram ?? null;
		this.hasShippingService = options.hasShippingService ?? null;
	}
}
