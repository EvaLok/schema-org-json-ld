<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class VacationRental extends TypedSchema {
	public const A_SCHEMA_TYPE = 'VacationRental';

	public function __construct(
		public string $name,
		public string $identifier,
		/** @var string[] $image */
		public array $image,
		public float $latitude,
		public float $longitude,
		public Accommodation $containsPlace,
		public null|string $additionalType = null,
		public null|PostalAddress $address = null,
		public null|AggregateRating $aggregateRating = null,
		public null|Brand $brand = null,
		public null|string $checkinTime = null,
		public null|string $checkoutTime = null,
		public null|string $description = null,
		/** @var string[] $knowsLanguage */
		public null|array $knowsLanguage = null,
		/** @var Review[] $review */
		public null|array $review = null,
	) {}
}
