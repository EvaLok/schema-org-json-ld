<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Offer extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Offer';

	/**
	 * @param OfferShippingDetails[]|null $shippingDetails
	 */
	public function __construct(
		public string $url,
		public string $priceCurrency,
		public float $price,
		public ItemAvailability $availability,
		public null|OfferItemCondition $itemCondition = null,
		public null|array $shippingDetails = null,
		public null|string $validFrom = null,
	) {}
}
