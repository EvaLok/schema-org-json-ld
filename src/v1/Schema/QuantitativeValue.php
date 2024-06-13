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
	 */
	function __construct(
		public float $value,
		public string $unitCode,
	){

	}
}
