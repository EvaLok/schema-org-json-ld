import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { BlogPosting } from "../../src/schema/BlogPosting";
import { ImageObject } from "../../src/schema/ImageObject";
import { Person } from "../../src/schema/Person";
import { WebPageElement } from "../../src/schema/WebPageElement";

describe("BlogPosting", () => {
	it("produces BlogPosting JSON-LD with required article fields", () => {
		const schema = new BlogPosting({ headline: "Blog headline" });
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("BlogPosting");
		expect(obj.headline).toBe("Blog headline");
	});

	it("omits inherited optional fields when null", () => {
		const schema = new BlogPosting({
			headline: "Blog headline",
			author: null,
			datePublished: null,
			dateModified: null,
			image: null,
			description: null,
			publisher: null,
			speakable: null,
			isAccessibleForFree: null,
			hasPart: null,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj).not.toHaveProperty("author");
		expect(obj).not.toHaveProperty("image");
		expect(obj).not.toHaveProperty("hasPart");
	});

	it("inherits full Article options while preserving BlogPosting type", () => {
		const schema = new BlogPosting({
			headline: "Blog headline",
			author: new Person({ name: "Jane" }),
			image: [
				"https://example.com/image.jpg",
				new ImageObject({ contentUrl: "https://example.com/image-object.jpg" }),
			],
			hasPart: new WebPageElement(true, ".content"),
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const image = obj.image as (string | Record<string, unknown>)[];

		expect(obj["@type"]).toBe("BlogPosting");
		expect(image[0]).toBe("https://example.com/image.jpg");
		expect((image[1] as Record<string, unknown>)["@type"]).toBe("ImageObject");
	});
});
