<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class SoftwareApplication extends TypedSchema {
	public const A_SCHEMA_TYPE = 'SoftwareApplication';

	public function __construct(
		public string $name,
		/** @var Offer[] $offers */
		public Offer|array $offers,
		public null|AggregateRating $aggregateRating,
		public null|string $applicationCategory = null,
		public null|string $operatingSystem = null,
		public null|Review $review = null,
		public null|string $description = null,
		public null|string $screenshot = null,
	) {}
}
