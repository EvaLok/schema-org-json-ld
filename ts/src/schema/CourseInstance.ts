import { TypedSchema } from "../TypedSchema.js";
import type { Person } from "./Person.js";
import type { Schedule } from "./Schedule.js";

export class CourseInstance extends TypedSchema {
	static readonly schemaType = "CourseInstance";

	constructor(
		public readonly courseMode: string | null = null,
		public readonly instructor: Person | null = null,
		public readonly courseSchedule: Schedule | null = null,
		public readonly courseWorkload: string | null = null,
	) {
		super();
	}
}
