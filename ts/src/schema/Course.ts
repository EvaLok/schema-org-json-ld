import { TypedSchema } from "../TypedSchema.js";
import type { AggregateRating } from "./AggregateRating.js";
import type { CourseInstance } from "./CourseInstance.js";
import type { Offer } from "./Offer.js";
import type { Organization } from "./Organization.js";

export interface CourseOptions {
	name: string;
	description: string;
	provider?: Organization | null;
	offers?: readonly Offer[] | null;
	hasCourseInstance?: readonly CourseInstance[] | null;
	courseCode?: string | null;
	inLanguage?: string | null;
	totalHistoricalEnrollment?: number | null;
	aggregateRating?: AggregateRating | null;
	image?: string | null;
}

export class Course extends TypedSchema {
	static readonly schemaType = "Course";

	public readonly name: string;
	public readonly description: string;
	public readonly provider: Organization | null;
	public readonly offers: readonly Offer[] | null;
	public readonly hasCourseInstance: readonly CourseInstance[] | null;
	public readonly courseCode: string | null;
	public readonly inLanguage: string | null;
	public readonly totalHistoricalEnrollment: number | null;
	public readonly aggregateRating: AggregateRating | null;
	public readonly image: string | null;

	constructor(options: CourseOptions) {
		super();
		this.name = options.name;
		this.description = options.description;
		this.provider = options.provider ?? null;
		this.offers = options.offers ?? null;
		this.hasCourseInstance = options.hasCourseInstance ?? null;
		this.courseCode = options.courseCode ?? null;
		this.inLanguage = options.inLanguage ?? null;
		this.totalHistoricalEnrollment = options.totalHistoricalEnrollment ?? null;
		this.aggregateRating = options.aggregateRating ?? null;
		this.image = options.image ?? null;
	}
}
