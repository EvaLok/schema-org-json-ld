<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ShippingRateSettings extends TypedSchema {
	public const A_SCHEMA_TYPE = 'ShippingRateSettings';

	public function __construct(
		public null|float $orderPercentage = null,
		public null|float $weightPercentage = null,
	) {}
}
