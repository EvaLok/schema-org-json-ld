import { TypedSchema } from "../TypedSchema.js";
import type { DataCatalog } from "./DataCatalog.js";
import type { DataDownload } from "./DataDownload.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";
import type { Place } from "./Place.js";

export interface DatasetOptions {
	name: string;
	description: string;
	url?: string | null;
	sameAs?: string | null;
	creator?: Person | Organization | null;
	funder?: Person | Organization | null;
	license?: string | null;
	keywords?: readonly string[] | null;
	identifier?: readonly string[] | null;
	isAccessibleForFree?: boolean | null;
	temporalCoverage?: string | null;
	spatialCoverage?: Place | null;
	includedInDataCatalog?: DataCatalog | null;
	distribution?: readonly DataDownload[] | null;
	variableMeasured?: string | null;
	measurementTechnique?: string | null;
	version?: string | null;
	alternateName?: string | null;
	citation?: string | null;
}

export class Dataset extends TypedSchema {
	static readonly schemaType = "Dataset";

	public readonly name: string;
	public readonly description: string;
	public readonly url: string | null;
	public readonly sameAs: string | null;
	public readonly creator: Person | Organization | null;
	public readonly funder: Person | Organization | null;
	public readonly license: string | null;
	public readonly keywords: readonly string[] | null;
	public readonly identifier: readonly string[] | null;
	public readonly isAccessibleForFree: boolean | null;
	public readonly temporalCoverage: string | null;
	public readonly spatialCoverage: Place | null;
	public readonly includedInDataCatalog: DataCatalog | null;
	public readonly distribution: readonly DataDownload[] | null;
	public readonly variableMeasured: string | null;
	public readonly measurementTechnique: string | null;
	public readonly version: string | null;
	public readonly alternateName: string | null;
	public readonly citation: string | null;

	constructor(options: DatasetOptions) {
		super();
		this.name = options.name;
		this.description = options.description;
		this.url = options.url ?? null;
		this.sameAs = options.sameAs ?? null;
		this.creator = options.creator ?? null;
		this.funder = options.funder ?? null;
		this.license = options.license ?? null;
		this.keywords = options.keywords ?? null;
		this.identifier = options.identifier ?? null;
		this.isAccessibleForFree = options.isAccessibleForFree ?? null;
		this.temporalCoverage = options.temporalCoverage ?? null;
		this.spatialCoverage = options.spatialCoverage ?? null;
		this.includedInDataCatalog = options.includedInDataCatalog ?? null;
		this.distribution = options.distribution ?? null;
		this.variableMeasured = options.variableMeasured ?? null;
		this.measurementTechnique = options.measurementTechnique ?? null;
		this.version = options.version ?? null;
		this.alternateName = options.alternateName ?? null;
		this.citation = options.citation ?? null;
	}
}
