import { LocalBusiness } from "./LocalBusiness.js";
import type { LocalBusinessOptions } from "./LocalBusiness.js";

export interface StoreOptions extends LocalBusinessOptions {}

export class Store extends LocalBusiness {
	static readonly schemaType = "Store";
}
