import { TypedSchema } from "../TypedSchema.js";
import type { Person } from "./Person.js";
import type { Schedule } from "./Schedule.js";

export interface CourseInstanceOptions {
	courseMode?: string | null;
	instructor?: Person | null;
	courseSchedule?: Schedule | null;
	courseWorkload?: string | null;
}

export class CourseInstance extends TypedSchema {
	static readonly schemaType = "CourseInstance";

	public readonly courseMode: string | null;
	public readonly instructor: Person | null;
	public readonly courseSchedule: Schedule | null;
	public readonly courseWorkload: string | null;

	constructor(options: CourseInstanceOptions) {
		super();
		this.courseMode = options.courseMode ?? null;
		this.instructor = options.instructor ?? null;
		this.courseSchedule = options.courseSchedule ?? null;
		this.courseWorkload = options.courseWorkload ?? null;
	}
}
