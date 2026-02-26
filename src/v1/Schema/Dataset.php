<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Dataset extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Dataset';

	public function __construct(
		public string $name,
		public string $description,
		public null|string $url = null,
		public null|string $sameAs = null,
		public null|Person|Organization $creator = null,
		public null|Person|Organization $funder = null,
		public null|string $license = null,
		/** @var string[] $keywords */
		public null|array $keywords = null,
		/** @var string[] $identifier */
		public null|array $identifier = null,
		public null|bool $isAccessibleForFree = null,
		public null|string $temporalCoverage = null,
		public null|Place $spatialCoverage = null,
		public null|DataCatalog $includedInDataCatalog = null,
		/** @var DataDownload[] $distribution */
		public null|array $distribution = null,
		public null|string $variableMeasured = null,
		public null|string $measurementTechnique = null,
		public null|string $version = null,
		public null|string $alternateName = null,
		public null|string $citation = null,
	) {}
}
