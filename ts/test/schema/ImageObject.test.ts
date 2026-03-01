import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ImageObject } from "../../src/schema/ImageObject";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";

describe("ImageObject", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new ImageObject({
			contentUrl: "https://example.com/image.jpg",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ImageObject");
		expect(obj.contentUrl).toBe("https://example.com/image.jpg");
	});

	it("omits optional fields when null", () => {
		const schema = new ImageObject({
			contentUrl: "https://example.com/image.jpg",
			url: null,
			name: null,
			caption: null,
			description: null,
			width: null,
			height: null,
			license: null,
			acquireLicensePage: null,
			creditText: null,
			copyrightNotice: null,
			creator: null,
			datePublished: null,
			uploadDate: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("creator");
		expect(obj).not.toHaveProperty("caption");
		expect(obj).not.toHaveProperty("uploadDate");
	});

	it("includes optional fields with Organization creator when set", () => {
		const schema = new ImageObject({
			contentUrl: "https://example.com/image.jpg",
			url: "https://example.com/image-page",
			name: "Product image",
			caption: "Front view",
			description: "High-resolution image",
			width: "1200",
			height: "800",
			license: "https://example.com/license",
			acquireLicensePage: "https://example.com/acquire",
			creditText: "Photo by Studio",
			copyrightNotice: "© Example Corp",
			creator: new Organization({ name: "Example Studio" }),
			datePublished: "2026-03-01",
			uploadDate: "2026-03-01",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const creator = obj.creator as Record<string, unknown>;

		expect(obj.name).toBe("Product image");
		expect(obj.caption).toBe("Front view");
		expect(creator["@type"]).toBe("Organization");
		expect(obj.uploadDate).toBe("2026-03-01");
	});

	it("supports Person creator", () => {
		const schema = new ImageObject({
			contentUrl: "https://example.com/image.jpg",
			creator: new Person({ name: "Jane Doe" }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const creator = obj.creator as Record<string, unknown>;

		expect(creator["@type"]).toBe("Person");
		expect(creator.name).toBe("Jane Doe");
	});
});
