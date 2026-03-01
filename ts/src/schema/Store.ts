import { LocalBusiness } from "./LocalBusiness.js";
import type { LocalBusinessOptions } from "./LocalBusiness.js";

export class Store extends LocalBusiness {
	static readonly schemaType = "Store";

	constructor(options: LocalBusinessOptions) {
		super(options);
	}
}
