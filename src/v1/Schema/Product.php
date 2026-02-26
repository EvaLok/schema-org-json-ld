<?php

declare(strict_types=1);

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
		public null|string $color = null,
		public null|string $material = null,
		public null|string $pattern = null,
		public null|string|SizeSpecification $size = null,
		public null|string $inProductGroupWithID = null,
		public null|string $gtin = null,
		public null|string $gtin8 = null,
		public null|string $gtin12 = null,
		public null|string $gtin13 = null,
		public null|string $gtin14 = null,
		public null|string $isbn = null,
		public null|ProductGroup $isVariantOf = null,
		public null|PeopleAudience $audience = null,
		/** @var null|Certification|Certification[] $hasCertification */
		public null|Certification|array $hasCertification = null,
	) {}
}
