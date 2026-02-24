<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class MonetaryAmount extends TypedSchema {
	public const A_SCHEMA_TYPE = 'MonetaryAmount';

	public function __construct(
		public string $currency,
		public null|float $value = null,
		public null|float $minValue = null,
		public null|float $maxValue = null,
	) {}

}
