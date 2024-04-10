<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Offer extends TypedSchema {
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
