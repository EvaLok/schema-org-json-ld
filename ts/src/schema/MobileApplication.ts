import { SoftwareApplication } from "./SoftwareApplication.js";
import type { SoftwareApplicationOptions } from "./SoftwareApplication.js";

export class MobileApplication extends SoftwareApplication {
	static readonly schemaType = "MobileApplication";

	constructor(options: SoftwareApplicationOptions) {
		super(options);
	}
}
