import { TypedSchema } from "../TypedSchema.js";

export interface NutritionInformationOptions {
	calories?: string | null;
	fatContent?: string | null;
	saturatedFatContent?: string | null;
	cholesterolContent?: string | null;
	sodiumContent?: string | null;
	carbohydrateContent?: string | null;
	fiberContent?: string | null;
	sugarContent?: string | null;
	proteinContent?: string | null;
	servingSize?: string | null;
}

export class NutritionInformation extends TypedSchema {
	static readonly schemaType = "NutritionInformation";

	public readonly calories: string | null;
	public readonly fatContent: string | null;
	public readonly saturatedFatContent: string | null;
	public readonly cholesterolContent: string | null;
	public readonly sodiumContent: string | null;
	public readonly carbohydrateContent: string | null;
	public readonly fiberContent: string | null;
	public readonly sugarContent: string | null;
	public readonly proteinContent: string | null;
	public readonly servingSize: string | null;

	constructor(options: NutritionInformationOptions = {}) {
		super();
		this.calories = options.calories ?? null;
		this.fatContent = options.fatContent ?? null;
		this.saturatedFatContent = options.saturatedFatContent ?? null;
		this.cholesterolContent = options.cholesterolContent ?? null;
		this.sodiumContent = options.sodiumContent ?? null;
		this.carbohydrateContent = options.carbohydrateContent ?? null;
		this.fiberContent = options.fiberContent ?? null;
		this.sugarContent = options.sugarContent ?? null;
		this.proteinContent = options.proteinContent ?? null;
		this.servingSize = options.servingSize ?? null;
	}
}
