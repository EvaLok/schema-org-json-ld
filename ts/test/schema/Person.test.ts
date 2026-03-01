import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { InteractionCounter } from "../../src/schema/InteractionCounter";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";
import { PostalAddress } from "../../src/schema/PostalAddress";

describe("Person", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Person({ name: "Jane Doe" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Person");
		expect(obj.name).toBe("Jane Doe");
	});

	it("omits optional fields when null", () => {
		const schema = new Person({
			name: "Jane Doe",
			url: null,
			image: null,
			email: null,
			telephone: null,
			jobTitle: null,
			worksFor: null,
			sameAs: null,
			description: null,
			givenName: null,
			familyName: null,
			address: null,
			interactionStatistic: null,
			agentInteractionStatistic: null,
			identifier: null,
			alternateName: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("worksFor");
		expect(obj).not.toHaveProperty("interactionStatistic");
		expect(obj).not.toHaveProperty("agentInteractionStatistic");
		expect(obj).not.toHaveProperty("address");
		expect(obj).not.toHaveProperty("alternateName");
	});

	it("includes optional fields when set", () => {
		const schema = new Person({
			name: "Jane Doe",
			url: "https://example.com/jane",
			jobTitle: "Engineer",
			worksFor: new Organization({ name: "Example Inc." }),
			sameAs: ["https://x.com/jane"],
			description: "Technical writer",
			givenName: "Jane",
			familyName: "Doe",
			address: new PostalAddress({
				streetAddress: "1600 Amphitheatre Parkway",
			}),
			interactionStatistic: new InteractionCounter("LikeAction", 12),
			agentInteractionStatistic: new InteractionCounter("FollowAction", 42),
			identifier: "person-123",
			alternateName: "J. Doe",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const worksFor = obj.worksFor as Record<string, unknown>;
		const interactionStatistic = obj.interactionStatistic as Record<
			string,
			unknown
		>;
		const agentInteractionStatistic = obj.agentInteractionStatistic as Record<
			string,
			unknown
		>;

		expect(obj.url).toBe("https://example.com/jane");
		expect(obj.jobTitle).toBe("Engineer");
		expect(worksFor["@type"]).toBe("Organization");
		expect(interactionStatistic["@type"]).toBe("InteractionCounter");
		expect(agentInteractionStatistic["@type"]).toBe("InteractionCounter");
		expect(obj.identifier).toBe("person-123");
	});

	it("supports interactionStatistic as array", () => {
		const schema = new Person({
			name: "Jane Doe",
			interactionStatistic: [
				new InteractionCounter("LikeAction", 12),
				new InteractionCounter("ShareAction", 3),
			],
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const interactionStatistic = obj.interactionStatistic as Record<
			string,
			unknown
		>[];

		expect(interactionStatistic).toHaveLength(2);
		expect(interactionStatistic[0]?.["@type"]).toBe("InteractionCounter");
		expect(interactionStatistic[1]?.["@type"]).toBe("InteractionCounter");
	});
});
