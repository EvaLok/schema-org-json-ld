<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ShippingConditions extends TypedSchema {
	public const A_SCHEMA_TYPE = 'ShippingConditions';

	public function __construct(
		public null|bool $doesNotShip = null,
		public null|QuantitativeValue $numItems = null,
		public null|MonetaryAmount $orderValue = null,
		public null|DefinedRegion $shippingDestination = null,
		public null|DefinedRegion $shippingOrigin = null,
		public null|OpeningHoursSpecification $seasonalOverride = null,
		public null|ShippingRateSettings|MonetaryAmount $shippingRate = null,
		public null|ServicePeriod $transitTime = null,
		public null|QuantitativeValue $weight = null,
	) {}
}
