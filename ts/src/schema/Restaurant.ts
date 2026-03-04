import { FoodEstablishment } from "./FoodEstablishment.js";
import type { FoodEstablishmentOptions } from "./FoodEstablishment.js";

export interface RestaurantOptions extends FoodEstablishmentOptions {}

export class Restaurant extends FoodEstablishment {
	static readonly schemaType = "Restaurant";
}
