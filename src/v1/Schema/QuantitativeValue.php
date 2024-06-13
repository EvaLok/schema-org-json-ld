<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class QuantitativeValue extends TypedSchema {
	const A_SCHEMA_TYPE = 'QuantitativeValue';

	/**
	 * @param $unitCode
	 * unitCode ref: https://github.com/schemaorg/schemaorg/wiki/Using-UN-CEFACT-Codes
	 * - kilogram: KGM
	 * - US pound: LBR
	 * - day: DAY
	 */
	function __construct(
		public null|float $value = null,
		public null|string $unitCode = null,
		public null|float $minValue = null,
		public null|float $maxValue = null,
	){

	}
}
