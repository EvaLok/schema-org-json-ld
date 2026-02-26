<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\Enum\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Enum\OfferItemCondition;
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
		public null|string $priceValidUntil = null,
		/** @var null|UnitPriceSpecification|UnitPriceSpecification[] $priceSpecification */
		public null|UnitPriceSpecification|array $priceSpecification = null,
	) {}
}
