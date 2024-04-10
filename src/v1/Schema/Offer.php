<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

class Offer {
	const A_SCHEMA_TYPE = 'Offer';

	public function __construct(
		public string $url,
		public string $priceCurrency,
		public float $price,
		public OfferItemCondition $itemCondition,
		public ItemAvailability $availability,
	) {

	}
}
