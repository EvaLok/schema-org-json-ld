import { SoftwareApplication } from "./SoftwareApplication.js";
import type { SoftwareApplicationOptions } from "./SoftwareApplication.js";

export interface WebApplicationOptions extends SoftwareApplicationOptions {}

export class WebApplication extends SoftwareApplication {
	static readonly schemaType = "WebApplication";
}
