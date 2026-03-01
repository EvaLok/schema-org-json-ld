import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { BroadcastEvent } from "../../src/schema/BroadcastEvent";
import { Clip } from "../../src/schema/Clip";
import { InteractionCounter } from "../../src/schema/InteractionCounter";
import { VideoObject } from "../../src/schema/VideoObject";

describe("VideoObject", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new VideoObject({
			name: "My Video",
			thumbnailUrl: ["https://example.com/thumb.jpg"],
			uploadDate: "2026-02-28",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("VideoObject");
		expect(obj.name).toBe("My Video");
		expect(obj.thumbnailUrl).toEqual(["https://example.com/thumb.jpg"]);
		expect(obj.uploadDate).toBe("2026-02-28");
	});

	it("omits optional fields when null", () => {
		const schema = new VideoObject({
			name: "My Video",
			thumbnailUrl: ["https://example.com/thumb.jpg"],
			uploadDate: "2026-02-28",
			description: null,
			contentUrl: null,
			embedUrl: null,
			duration: null,
			expires: null,
			regionsAllowed: null,
			interactionStatistic: null,
			hasPart: null,
			ineligibleRegion: null,
			publication: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("description");
		expect(obj).not.toHaveProperty("contentUrl");
		expect(obj).not.toHaveProperty("embedUrl");
		expect(obj).not.toHaveProperty("duration");
		expect(obj).not.toHaveProperty("expires");
		expect(obj).not.toHaveProperty("regionsAllowed");
		expect(obj).not.toHaveProperty("interactionStatistic");
		expect(obj).not.toHaveProperty("hasPart");
		expect(obj).not.toHaveProperty("ineligibleRegion");
		expect(obj).not.toHaveProperty("publication");
	});

	it("includes optional fields when set", () => {
		const schema = new VideoObject({
			name: "My Video",
			thumbnailUrl: [
				"https://example.com/thumb-1.jpg",
				"https://example.com/thumb-2.jpg",
			],
			uploadDate: "2026-02-28",
			description: "A tutorial video",
			contentUrl: "https://example.com/video.mp4",
			embedUrl: "https://example.com/embed/video",
			duration: "PT2M",
			expires: "2026-12-31",
			regionsAllowed: "US",
			interactionStatistic: new InteractionCounter("WatchAction", 123),
			hasPart: [new Clip("Intro", 0, "https://example.com/video#t=0", 30)],
			ineligibleRegion: "CA",
			publication: new BroadcastEvent(true, "2026-02-28T10:00:00Z"),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const interactionStatistic = obj.interactionStatistic as Record<
			string,
			unknown
		>;
		const hasPart = obj.hasPart as Record<string, unknown>[];
		const publication = obj.publication as Record<string, unknown>;

		expect(obj.description).toBe("A tutorial video");
		expect(obj.contentUrl).toBe("https://example.com/video.mp4");
		expect(obj.embedUrl).toBe("https://example.com/embed/video");
		expect(obj.duration).toBe("PT2M");
		expect(obj.expires).toBe("2026-12-31");
		expect(obj.regionsAllowed).toBe("US");
		expect(interactionStatistic["@type"]).toBe("InteractionCounter");
		expect(hasPart[0]?.["@type"]).toBe("Clip");
		expect(publication["@type"]).toBe("BroadcastEvent");
		expect(obj.ineligibleRegion).toBe("CA");
	});

	it("supports interactionStatistic as array", () => {
		const schema = new VideoObject({
			name: "My Video",
			thumbnailUrl: ["https://example.com/thumb.jpg"],
			uploadDate: "2026-02-28",
			interactionStatistic: [
				new InteractionCounter("WatchAction", 123),
				new InteractionCounter("LikeAction", 10),
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
