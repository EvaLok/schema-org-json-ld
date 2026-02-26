<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

class FoodEstablishment extends LocalBusiness {
	public const A_SCHEMA_TYPE = 'FoodEstablishment';

	public function __construct(
		string $name,
		PostalAddress $address,
		null|string $url = null,
		null|string $telephone = null,
		null|string $description = null,
		/** @var string[] $image */
		null|array $image = null,
		null|string $priceRange = null,
		null|GeoCoordinates $geo = null,
		/** @var OpeningHoursSpecification[] $openingHoursSpecification */
		null|array $openingHoursSpecification = null,
		null|AggregateRating $aggregateRating = null,
		/** @var Review|Review[] $review */
		null|Review|array $review = null,
		null|string $menu = null,
		null|string $servesCuisine = null,
		null|string $logo = null,
		null|string $email = null,
		/** @var string[] $sameAs */
		null|array $sameAs = null,
		/** @var LocalBusiness[] $department */
		null|LocalBusiness|array $department = null,
		public null|bool|string $acceptsReservations = null,
	) {
		parent::__construct(
			name: $name,
			address: $address,
			url: $url,
			telephone: $telephone,
			description: $description,
			image: $image,
			priceRange: $priceRange,
			geo: $geo,
			openingHoursSpecification: $openingHoursSpecification,
			aggregateRating: $aggregateRating,
			review: $review,
			menu: $menu,
			servesCuisine: $servesCuisine,
			logo: $logo,
			email: $email,
			sameAs: $sameAs,
			department: $department,
		);
	}
}
