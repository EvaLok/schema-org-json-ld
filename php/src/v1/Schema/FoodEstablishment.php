<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

class FoodEstablishment extends LocalBusiness {
	public const A_SCHEMA_TYPE = 'FoodEstablishment';

	/**
	 * @param string[]|null $image
	 * @param OpeningHoursSpecification[]|null $openingHoursSpecification
	 * @param Review|Review[]|null $review
	 * @param string[]|null $sameAs
	 * @param LocalBusiness|LocalBusiness[]|null $department
	 */
	public function __construct(
		string $name,
		PostalAddress $address,
		null|string $url = null,
		null|string $telephone = null,
		null|string $description = null,
		null|array $image = null,
		null|string $priceRange = null,
		null|GeoCoordinates $geo = null,
		null|array $openingHoursSpecification = null,
		null|AggregateRating $aggregateRating = null,
		null|Review|array $review = null,
		null|string $menu = null,
		null|string $servesCuisine = null,
		null|string $logo = null,
		null|string $email = null,
		null|array $sameAs = null,
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
