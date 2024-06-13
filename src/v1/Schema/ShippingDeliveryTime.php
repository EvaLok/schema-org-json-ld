<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class ShippingDeliveryTime extends TypedSchema {
	const A_SCHEMA_TYPE = 'ShippingDeliveryTime';

	function __construct(
		public QuantitativeValue $handlingTime,
		public QuantitativeValue $transitTime,
	) {

	}
}

