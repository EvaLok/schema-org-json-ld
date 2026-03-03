import { TypedSchema } from "../TypedSchema.js";

export interface ScheduleOptions {
	repeatFrequency: string;
	repeatCount?: number | null;
	startDate?: string | null;
	endDate?: string | null;
}

export class Schedule extends TypedSchema {
	static readonly schemaType = "Schedule";

	public readonly repeatFrequency: string;
	public readonly repeatCount: number | null;
	public readonly startDate: string | null;
	public readonly endDate: string | null;

	constructor(options: ScheduleOptions) {
		super();
		this.repeatFrequency = options.repeatFrequency;
		this.repeatCount = options.repeatCount ?? null;
		this.startDate = options.startDate ?? null;
		this.endDate = options.endDate ?? null;
	}
}
