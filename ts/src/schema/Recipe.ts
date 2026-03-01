import { TypedSchema } from "../TypedSchema.js";
import type { AggregateRating } from "./AggregateRating.js";
import type { BroadcastEvent } from "./BroadcastEvent.js";
import type { Clip } from "./Clip.js";
import type { HowToSection } from "./HowToSection.js";
import type { HowToStep } from "./HowToStep.js";
import type { InteractionCounter } from "./InteractionCounter.js";
import type { NutritionInformation } from "./NutritionInformation.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";
import type { Review } from "./Review.js";
import type { VideoObject } from "./VideoObject.js";

export interface RecipeOptions {
	name: string;
	image: readonly string[];
	author?: Person | Organization | null;
	datePublished?: string | null;
	description?: string | null;
	prepTime?: string | null;
	cookTime?: string | null;
	totalTime?: string | null;
	keywords?: string | null;
	recipeYield?: string | null;
	recipeCategory?: string | null;
	recipeCuisine?: string | null;
	recipeIngredient?: readonly string[] | null;
	recipeInstructions?: readonly (HowToStep | HowToSection)[] | null;
	nutrition?: NutritionInformation | null;
	aggregateRating?: AggregateRating | null;
	review?: Review | readonly Review[] | null;
	video?: VideoObject | null;
	expires?: string | null;
	hasPart?: readonly Clip[] | null;
	publication?: BroadcastEvent | null;
	ineligibleRegion?: string | null;
	interactionStatistic?:
		| InteractionCounter
		| readonly InteractionCounter[]
		| null;
}

export class Recipe extends TypedSchema {
	static readonly schemaType = "Recipe";

	public readonly name: string;
	public readonly image: readonly string[];
	public readonly author: Person | Organization | null;
	public readonly datePublished: string | null;
	public readonly description: string | null;
	public readonly prepTime: string | null;
	public readonly cookTime: string | null;
	public readonly totalTime: string | null;
	public readonly keywords: string | null;
	public readonly recipeYield: string | null;
	public readonly recipeCategory: string | null;
	public readonly recipeCuisine: string | null;
	public readonly recipeIngredient: readonly string[] | null;
	public readonly recipeInstructions:
		| readonly (HowToStep | HowToSection)[]
		| null;
	public readonly nutrition: NutritionInformation | null;
	public readonly aggregateRating: AggregateRating | null;
	public readonly review: Review | readonly Review[] | null;
	public readonly video: VideoObject | null;
	public readonly expires: string | null;
	public readonly hasPart: readonly Clip[] | null;
	public readonly publication: BroadcastEvent | null;
	public readonly ineligibleRegion: string | null;
	public readonly interactionStatistic:
		| InteractionCounter
		| readonly InteractionCounter[]
		| null;

	constructor(options: RecipeOptions) {
		super();
		this.name = options.name;
		this.image = options.image;
		this.author = options.author ?? null;
		this.datePublished = options.datePublished ?? null;
		this.description = options.description ?? null;
		this.prepTime = options.prepTime ?? null;
		this.cookTime = options.cookTime ?? null;
		this.totalTime = options.totalTime ?? null;
		this.keywords = options.keywords ?? null;
		this.recipeYield = options.recipeYield ?? null;
		this.recipeCategory = options.recipeCategory ?? null;
		this.recipeCuisine = options.recipeCuisine ?? null;
		this.recipeIngredient = options.recipeIngredient ?? null;
		this.recipeInstructions = options.recipeInstructions ?? null;
		this.nutrition = options.nutrition ?? null;
		this.aggregateRating = options.aggregateRating ?? null;
		this.review = options.review ?? null;
		this.video = options.video ?? null;
		this.expires = options.expires ?? null;
		this.hasPart = options.hasPart ?? null;
		this.publication = options.publication ?? null;
		this.ineligibleRegion = options.ineligibleRegion ?? null;
		this.interactionStatistic = options.interactionStatistic ?? null;
	}
}
