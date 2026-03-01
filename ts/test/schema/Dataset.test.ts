import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { DataCatalog } from "../../src/schema/DataCatalog";
import { DataDownload } from "../../src/schema/DataDownload";
import { Dataset } from "../../src/schema/Dataset";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";
import { Place } from "../../src/schema/Place";

describe("Dataset", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Dataset({
			name: "Product Catalog Dataset",
			description: "Dataset containing product records.",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Dataset");
		expect(obj.name).toBe("Product Catalog Dataset");
		expect(obj.description).toBe("Dataset containing product records.");
	});

	it("omits optional fields when null", () => {
		const schema = new Dataset({
			name: "Product Catalog Dataset",
			description: "Dataset containing product records.",
			url: null,
			sameAs: null,
			creator: null,
			funder: null,
			license: null,
			keywords: null,
			identifier: null,
			isAccessibleForFree: null,
			temporalCoverage: null,
			spatialCoverage: null,
			includedInDataCatalog: null,
			distribution: null,
			variableMeasured: null,
			measurementTechnique: null,
			version: null,
			alternateName: null,
			citation: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("creator");
		expect(obj).not.toHaveProperty("funder");
		expect(obj).not.toHaveProperty("keywords");
		expect(obj).not.toHaveProperty("distribution");
		expect(obj).not.toHaveProperty("citation");
	});

	it("includes representative optional fields when set", () => {
		const schema = new Dataset({
			name: "Product Catalog Dataset",
			description: "Dataset containing product records.",
			url: "https://example.com/datasets/products",
			sameAs: "https://doi.org/10.1234/example-dataset",
			creator: new Person({ name: "Jane Doe" }),
			funder: new Organization({ name: "Example Foundation" }),
			license: "https://example.com/license",
			keywords: ["products", "catalog"],
			identifier: ["dataset-123", "doi:10.1234/example-dataset"],
			isAccessibleForFree: true,
			temporalCoverage: "2024-01-01/2026-01-01",
			spatialCoverage: new Place("Global"),
			includedInDataCatalog: new DataCatalog("Example Data Catalog"),
			distribution: [
				new DataDownload("https://example.com/datasets/products.csv", "text/csv"),
			],
			variableMeasured: "price",
			measurementTechnique: "survey",
			version: "1.0",
			alternateName: "Products dataset",
			citation: "Example Citation",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const creator = obj.creator as Record<string, unknown>;
		const funder = obj.funder as Record<string, unknown>;
		const spatialCoverage = obj.spatialCoverage as Record<string, unknown>;
		const includedInDataCatalog = obj.includedInDataCatalog as Record<
			string,
			unknown
		>;
		const distribution = obj.distribution as Record<string, unknown>[];

		expect(obj.url).toBe("https://example.com/datasets/products");
		expect(creator["@type"]).toBe("Person");
		expect(funder["@type"]).toBe("Organization");
		expect(obj.keywords).toEqual(["products", "catalog"]);
		expect(obj.identifier).toEqual(["dataset-123", "doi:10.1234/example-dataset"]);
		expect(spatialCoverage["@type"]).toBe("Place");
		expect(includedInDataCatalog["@type"]).toBe("DataCatalog");
		expect(distribution[0]?.["@type"]).toBe("DataDownload");
	});

	it("supports Organization creator and Person funder", () => {
		const schema = new Dataset({
			name: "Product Catalog Dataset",
			description: "Dataset containing product records.",
			creator: new Organization({ name: "Example Corp" }),
			funder: new Person({ name: "John Smith" }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const creator = obj.creator as Record<string, unknown>;
		const funder = obj.funder as Record<string, unknown>;

		expect(creator["@type"]).toBe("Organization");
		expect(funder["@type"]).toBe("Person");
	});
});
