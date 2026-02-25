<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class LocalBusiness extends TypedSchema {
	public const A_SCHEMA_TYPE = 'LocalBusiness';

	public function __construct(
		public string $name,
		public PostalAddress $address,
		public null|string $url = null,
		public null|string $telephone = null,
		public null|string $description = null,
		/** @var string[] $image */
		public null|array $image = null,
		public null|string $priceRange = null,
		public null|GeoCoordinates $geo = null,
		/** @var OpeningHoursSpecification[] $openingHoursSpecification */
		public null|array $openingHoursSpecification = null,
		public null|AggregateRating $aggregateRating = null,
		/** @var Review|Review[] $review */
		public null|Review|array $review = null,
		public null|string $menu = null,
		public null|string $servesCuisine = null,
		public null|string $logo = null,
		public null|string $email = null,
		/** @var string[] $sameAs */
		public null|array $sameAs = null,
		/** @var LocalBusiness[] $department */
		public null|LocalBusiness|array $department = null,
	) {}
}
