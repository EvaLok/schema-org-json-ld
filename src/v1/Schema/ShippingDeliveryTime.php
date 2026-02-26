<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ShippingDeliveryTime extends TypedSchema {
	public const A_SCHEMA_TYPE = 'ShippingDeliveryTime';

	public function __construct(
		public QuantitativeValue $handlingTime,
		public QuantitativeValue $transitTime,
	) {}
}
