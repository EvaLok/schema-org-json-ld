<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ProductGroup extends TypedSchema {
	public const A_SCHEMA_TYPE = 'ProductGroup';

	public function __construct(
		public string $name,
		public null|string $productGroupID = null,
		/** @var null|string|string[] $variesBy */
		public null|string|array $variesBy = null,
		/** @var null|Product|Product[] $hasVariant */
		public null|Product|array $hasVariant = null,
		public null|string $url = null,
		public null|string $description = null,
		public null|Brand $brand = null,
		public null|AggregateRating $aggregateRating = null,
		public null|Review $review = null,
	) {}
}
