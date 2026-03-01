import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { CourseInstance } from "../../src/schema/CourseInstance";
import { Person } from "../../src/schema/Person";
import { Schedule } from "../../src/schema/Schedule";

describe("CourseInstance", () => {
	it("produces minimal JSON-LD output with all-null construction", () => {
		const schema = new CourseInstance();
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("CourseInstance");
	});

	it("omits optional fields when null", () => {
		const schema = new CourseInstance(null, null, null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("courseMode");
		expect(obj).not.toHaveProperty("instructor");
		expect(obj).not.toHaveProperty("courseSchedule");
		expect(obj).not.toHaveProperty("courseWorkload");
	});

	it("includes all fields when set", () => {
		const schema = new CourseInstance(
			"online",
			new Person({ name: "Jane Doe" }),
			new Schedule("P1W"),
			"PT5H",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const instructor = obj.instructor as Record<string, unknown>;
		const courseSchedule = obj.courseSchedule as Record<string, unknown>;

		expect(obj.courseMode).toBe("online");
		expect(instructor["@type"]).toBe("Person");
		expect(courseSchedule["@type"]).toBe("Schedule");
		expect(obj.courseWorkload).toBe("PT5H");
	});
});
