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
		public OfferItemCondition $itemCondition,
		public ItemAvailability $availability,
		public null|array $shippingDetails = null,
	) {}
}
