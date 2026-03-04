import { SoftwareApplication } from "./SoftwareApplication.js";
import type { SoftwareApplicationOptions } from "./SoftwareApplication.js";

export interface MobileApplicationOptions extends SoftwareApplicationOptions {}

export class MobileApplication extends SoftwareApplication {
	static readonly schemaType = "MobileApplication";
}
