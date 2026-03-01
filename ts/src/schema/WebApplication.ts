import { SoftwareApplication } from "./SoftwareApplication.js";
import type { SoftwareApplicationOptions } from "./SoftwareApplication.js";

export class WebApplication extends SoftwareApplication {
	static readonly schemaType = "WebApplication";

	constructor(options: SoftwareApplicationOptions) {
		super(options);
	}
}
