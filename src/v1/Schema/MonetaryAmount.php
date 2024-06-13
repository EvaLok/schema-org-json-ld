<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class MonetaryAmount extends TypedSchema {
	const A_SCHEMA_TYPE = 'MonetaryAmount';

	function __construct(
		public string $currency,
		public null|float $value = null,
		public null|float $minValue = null,
		public null|float $maxValue = null,
	) {

	}

}
