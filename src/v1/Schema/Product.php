<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Product extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Product';

	public function __construct(
		public string $name,
		/** @var string[] $image */
		public array  $image,
		public string $description,
		public string $sku,
		/** @var Offer[]|AggregateOffer $offers */
		public array|AggregateOffer  $offers,
		public null|Brand $brand = null,
		public null|string $mpn = null,
		public null|QuantitativeValue $weight = null,
		public null|AggregateRating $aggregateRating = null,
		/** @var null|Review|Review[] $review */
		public null|Review|array $review = null,
	) {}
}
