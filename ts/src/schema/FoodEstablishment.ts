import { LocalBusiness } from "./LocalBusiness.js";
import type { LocalBusinessOptions } from "./LocalBusiness.js";

export interface FoodEstablishmentOptions extends LocalBusinessOptions {
	acceptsReservations?: boolean | string | null;
}

export class FoodEstablishment extends LocalBusiness {
	static readonly schemaType: string = "FoodEstablishment";

	public readonly acceptsReservations: boolean | string | null;

	constructor(options: FoodEstablishmentOptions) {
		super(options);
		this.acceptsReservations = options.acceptsReservations ?? null;
	}
}
