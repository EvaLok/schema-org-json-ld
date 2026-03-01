import { TypedSchema } from "../TypedSchema.js";
import type { AdministrativeArea } from "./AdministrativeArea.js";
import type { MonetaryAmount } from "./MonetaryAmount.js";
import type { Organization } from "./Organization.js";
import type { Place } from "./Place.js";
import type { PropertyValue } from "./PropertyValue.js";

export interface JobPostingOptions {
	title: string;
	description: string;
	datePosted: string;
	hiringOrganization: Organization;
	jobLocation?: Place | null;
	baseSalary?: MonetaryAmount | null;
	employmentType?: string | null;
	validThrough?: string | null;
	applicantLocationRequirements?: AdministrativeArea | null;
	jobLocationType?: string | null;
	directApply?: boolean | null;
	identifier?: PropertyValue | null;
}

export class JobPosting extends TypedSchema {
	static readonly schemaType = "JobPosting";

	public readonly title: string;
	public readonly description: string;
	public readonly datePosted: string;
	public readonly hiringOrganization: Organization;
	public readonly jobLocation: Place | null;
	public readonly baseSalary: MonetaryAmount | null;
	public readonly employmentType: string | null;
	public readonly validThrough: string | null;
	public readonly applicantLocationRequirements: AdministrativeArea | null;
	public readonly jobLocationType: string | null;
	public readonly directApply: boolean | null;
	public readonly identifier: PropertyValue | null;

	constructor(options: JobPostingOptions) {
		super();
		this.title = options.title;
		this.description = options.description;
		this.datePosted = options.datePosted;
		this.hiringOrganization = options.hiringOrganization;
		this.jobLocation = options.jobLocation ?? null;
		this.baseSalary = options.baseSalary ?? null;
		this.employmentType = options.employmentType ?? null;
		this.validThrough = options.validThrough ?? null;
		this.applicantLocationRequirements =
			options.applicantLocationRequirements ?? null;
		this.jobLocationType = options.jobLocationType ?? null;
		this.directApply = options.directApply ?? null;
		this.identifier = options.identifier ?? null;
	}
}
